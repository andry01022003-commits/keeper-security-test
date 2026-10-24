//! Conda as the second consumer of the metadata revision cache (#4174).
//!
//! A hosted subdir's documents (`repodata.json` and its `.zst`/`.bz2`
//! encodings, `current_repodata.json`, `run_exports.json` and the bootstrap
//! `repodata.json.jlap`) are rendered together from one catalog walk and
//! kept under the subdir's revision ([`super::shard_store::subdir_state`]);
//! the CEP-16 shard index is kept the same way from the stored shard map, and
//! `channeldata.json` under the repository's revision. A request reads the
//! revision (one statement against the conda state tables, no catalog rows),
//! serves the kept bytes when it matches, and otherwise renders once under
//! the cache's per-`(repository, scope)` single-flight lock.
//!
//! The revision is the fingerprint, and it already covers quarantine state:
//! the catalog triggers mark a key dirty on every quarantine transition, and
//! the revision moves when the key is rebuilt, which every reader does before
//! it reads the revision. A lapsing hold moves it through
//! `next_hold_expiry`. Each render decides visibility at the database instant
//! the revision was read, so the bytes kept under a revision are the bytes
//! that revision describes.
//!
//! Other replicas learn of a rebuild through the conda invalidation channel
//! ([`crate::services::cache_invalidation::CONDA_REPODATA_INVALIDATION_CHANNEL`])
//! and drop their entries for the subdir; that frees memory early, while the
//! per-request revision check is what keeps every replica correct.

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::services::repo_metadata_revision_cache::{
    CachedRender, Fingerprint, RepoMetadataRevisionCache, RevisionCacheConfig,
};

use super::shard_store;

/// Subdirs kept warm per process.
const SUBDIR_CACHE_MAX_ENTRIES: usize = 64;
/// Byte budget of the subdir cache. A 30,000-record subdir renders about
/// 25 MiB of pretty `repodata.json` plus a few MiB of compressed encodings.
const SUBDIR_CACHE_MAX_BYTES: usize = 512 * 1024 * 1024;
/// Shard indexes and channeldata documents kept per process.
const SMALL_CACHE_MAX_ENTRIES: usize = 256;
const SMALL_CACHE_MAX_BYTES: usize = 128 * 1024 * 1024;

/// The fingerprint of a hosted conda scope: its revision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Revision(pub i64);

impl Fingerprint for Revision {}

/// One served document: bytes, their ETag, and a gzip rendering made on the
/// first request that asks for one.
pub(crate) struct Prepared {
    pub body: Bytes,
    pub etag: String,
    gzip: tokio::sync::OnceCell<Bytes>,
}

impl Prepared {
    pub fn new(body: impl Into<Bytes>) -> Self {
        let body: Bytes = body.into();
        let etag = crate::api::handlers::cache_headers::compute_etag(&body);
        Self {
            body,
            etag,
            gzip: tokio::sync::OnceCell::new(),
        }
    }

    fn weight(&self) -> usize {
        self.body.len() + self.gzip.get().map_or(0, Bytes::len)
    }

    /// The gzip rendering, compressed once on the blocking pool. `None` when
    /// the blocking task failed; the caller serves the identity body.
    pub async fn gzip(&self) -> Option<Bytes> {
        let body = self.body.clone();
        self.gzip
            .get_or_try_init(|| async move {
                tokio::task::spawn_blocking(move || Bytes::from(super::gzip_compress(&body))).await
            })
            .await
            .ok()
            .cloned()
    }
}

impl CachedRender for Prepared {
    fn weight_bytes(&self) -> usize {
        self.weight()
    }
}

/// Every document of one hosted subdir at one revision.
pub(crate) struct SubdirDocs {
    /// `repodata.json` as served (pretty-printed); what `.sig` signs and the
    /// JLAP footer hashes.
    pub json: Prepared,
    pub zst: Prepared,
    pub bz2: Prepared,
    pub current: Prepared,
    pub run_exports: Prepared,
    pub jlap: Prepared,
}

impl CachedRender for SubdirDocs {
    fn weight_bytes(&self) -> usize {
        self.json.weight()
            + self.zst.weight()
            + self.bz2.weight()
            + self.current.weight()
            + self.run_exports.weight()
            + self.jlap.weight()
    }
}

type SubdirCache = RepoMetadataRevisionCache<Revision, SubdirDocs>;
type DocCache = RepoMetadataRevisionCache<Revision, Prepared>;

fn config(scope_kind: &'static str, max_entries: usize, max_bytes: usize) -> RevisionCacheConfig {
    RevisionCacheConfig {
        format: "conda",
        scope_kind,
        max_entries,
        max_bytes,
        // A hosted document does not depend on the caller.
        max_partitions_per_scope: 1,
        lock_stripes: max_entries,
        incomplete_ttl: Duration::from_secs(60),
    }
}

pub(crate) fn subdir_cache() -> &'static SubdirCache {
    static CACHE: OnceLock<SubdirCache> = OnceLock::new();
    CACHE.get_or_init(|| {
        SubdirCache::with_config(config(
            "subdir",
            SUBDIR_CACHE_MAX_ENTRIES,
            SUBDIR_CACHE_MAX_BYTES,
        ))
    })
}

pub(crate) fn shard_index_cache() -> &'static DocCache {
    static CACHE: OnceLock<DocCache> = OnceLock::new();
    CACHE.get_or_init(|| {
        DocCache::with_config(config(
            "shard_index",
            SMALL_CACHE_MAX_ENTRIES,
            SMALL_CACHE_MAX_BYTES,
        ))
    })
}

pub(crate) fn channel_cache() -> &'static DocCache {
    static CACHE: OnceLock<DocCache> = OnceLock::new();
    CACHE.get_or_init(|| {
        DocCache::with_config(config(
            "channel",
            SMALL_CACHE_MAX_ENTRIES,
            SMALL_CACHE_MAX_BYTES,
        ))
    })
}

/// Scope of a repository's `channeldata.json` entry.
const CHANNEL_SCOPE: &str = "";

/// Drop this process's entries for `subdirs` of `repo_id` (and its
/// channel-wide document). Called by the rebuild that moved them and by the
/// invalidation listener for rebuilds on other replicas.
/// An empty `subdirs` drops every entry of the repository.
pub(crate) async fn invalidate_local(repo_id: Uuid, subdirs: &[String]) {
    if subdirs.is_empty() {
        subdir_cache().invalidate_repo(repo_id).await;
        shard_index_cache().invalidate_repo(repo_id).await;
        channel_cache().invalidate_repo(repo_id).await;
        return;
    }
    for subdir in subdirs {
        subdir_cache().invalidate_scope(repo_id, subdir).await;
        shard_index_cache().invalidate_scope(repo_id, subdir).await;
    }
    channel_cache()
        .invalidate_scope(repo_id, CHANNEL_SCOPE)
        .await;
}

fn render_failed(e: impl std::fmt::Display) -> Response {
    tracing::error!(error = %e, "conda repodata render failed");
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
}

/// The documents of hosted `(repo_id, subdir)` at its current revision.
pub(crate) async fn hosted_subdir_docs(
    db: &PgPool,
    repo_id: Uuid,
    subdir: &str,
) -> Result<Arc<SubdirDocs>, Response> {
    super::validate_read_subdir(subdir)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid subdir: {}", e)).into_response())?;
    let state = shard_store::subdir_state(db, repo_id, subdir).await?;
    let subdir_owned = subdir.to_string();
    subdir_cache()
        .get_or_render_scoped(repo_id, subdir, Revision(state.revision), || {
            render_subdir_docs(db.clone(), repo_id, subdir_owned, state.now)
        })
        .await
}

/// Render every document of a hosted subdir from one catalog walk, deciding
/// visibility at `now`.
async fn render_subdir_docs(
    db: PgPool,
    repo_id: Uuid,
    subdir: String,
    now: DateTime<Utc>,
) -> Result<SubdirDocs, Response> {
    let all = super::list_conda_artifacts_at(&db, repo_id, now).await?;
    let removed = super::list_removed_artifacts_at(&db, repo_id, &subdir, now).await?;
    tokio::task::spawn_blocking(move || -> Result<SubdirDocs, Response> {
        let in_subdir = super::artifacts_for_subdir(&all, &subdir);
        let full = super::repodata_document(&in_subdir, &subdir, false, &removed);
        let current = super::repodata_document(&in_subdir, &subdir, true, &removed);
        let run_exports = super::run_exports_document(&in_subdir, &subdir);
        let json = serde_json::to_vec_pretty(&full).map_err(render_failed)?;
        let zst = super::RepodataEncoding::Zst.encode(&full)?;
        let bz2 = super::RepodataEncoding::Bz2.encode(&full)?;
        let jlap = super::build_bootstrap_jlap(&json);
        Ok(SubdirDocs {
            json: Prepared::new(json),
            zst: Prepared::new(zst),
            bz2: Prepared::new(bz2),
            current: Prepared::new(serde_json::to_vec_pretty(&current).map_err(render_failed)?),
            run_exports: Prepared::new(
                serde_json::to_vec_pretty(&run_exports).map_err(render_failed)?,
            ),
            jlap: Prepared::new(jlap),
        })
    })
    .await
    .map_err(render_failed)?
}

/// The CEP-16 shard index of hosted `(repo_id, subdir)`, serialised from the
/// stored shard map, or `None` while the repository's backfill has not
/// finished (the caller keeps the per-request walk).
pub(crate) async fn hosted_shard_index(
    db: &PgPool,
    repo_id: Uuid,
    subdir: &str,
) -> Result<Option<Arc<Prepared>>, Response> {
    let state = shard_store::subdir_state(db, repo_id, subdir).await?;
    if !state.backfilled {
        return Ok(None);
    }
    let subdir_owned = subdir.to_string();
    shard_index_cache()
        .get_or_render_scoped(repo_id, subdir, Revision(state.revision), || async move {
            let shards = shard_store::stored_index(db, repo_id, &subdir_owned).await?;
            let body =
                super::serialize_msgpack_zst(&super::build_sharded_index(&subdir_owned, &shards))?;
            Ok::<_, Response>(Prepared::new(body))
        })
        .await
        .map(Some)
}

/// The stored shard at `address` of hosted `(repo_id, subdir)`. `Ok(None)`
/// while the backfill has not finished; `Ok(Some(None))` when no shard of the
/// subdir has that address.
pub(crate) async fn hosted_shard(
    db: &PgPool,
    repo_id: Uuid,
    subdir: &str,
    address: &[u8],
) -> Result<Option<Option<Vec<u8>>>, Response> {
    let state = shard_store::subdir_state(db, repo_id, subdir).await?;
    if !state.backfilled {
        return Ok(None);
    }
    shard_store::stored_shard(db, repo_id, subdir, address)
        .await
        .map(Some)
}

/// `channeldata.json` of hosted `repo_id` at its current revision.
pub(crate) async fn hosted_channeldata(
    db: &PgPool,
    repo_id: Uuid,
) -> Result<Arc<Prepared>, Response> {
    let state = shard_store::repo_state(db, repo_id).await?;
    channel_cache()
        .get_or_render_scoped(
            repo_id,
            CHANNEL_SCOPE,
            Revision(state.revision),
            || async move {
                let body = super::render_hosted_channeldata(db, repo_id, state.now).await?;
                Ok::<_, Response>(Prepared::new(body))
            },
        )
        .await
}
