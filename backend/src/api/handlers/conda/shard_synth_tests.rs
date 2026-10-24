//! Tests of CEP-16 shards synthesised for a Remote conda repository whose
//! upstream publishes no shard index (#4178).

use super::shard_synth::{self, for_each_record, shards_of_partition, spill_document};
use super::tests::{cleanup_conda_repo, insert_public_remote_conda_repo, state_with_proxy_cache};
use super::virtual_channel_tests::upstream_record;
use super::{shard_address, RepodataEncoding};
use crate::api::handlers::test_db_helpers as tdh;
use axum::http::StatusCode;
use std::io::Write;
use wiremock::matchers::{method, path as wm_path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// An upstream `repodata.json` with `.tar.bz2` records in `packages` and
/// `.conda` records in `packages.conda`, plus the members a real channel
/// carries around them.
fn repodata(subdir: &str, tar: &[&str], conda: &[&str]) -> serde_json::Value {
    let section = |files: &[&str]| -> serde_json::Map<String, serde_json::Value> {
        files
            .iter()
            .map(|f| (f.to_string(), upstream_record(subdir, f)))
            .collect()
    };
    serde_json::json!({
        "info": {"subdir": subdir, "base_url": "https://example.invalid/"},
        "packages": section(tar),
        "packages.conda": section(conda),
        "removed": ["gone-1.0-0.tar.bz2"],
        "repodata_version": 1,
    })
}

fn bz2(doc: &serde_json::Value) -> Vec<u8> {
    let mut enc = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::default());
    enc.write_all(&serde_json::to_vec(doc).unwrap()).unwrap();
    enc.finish().unwrap()
}

async fn mount(server: &MockServer, path: &str, body: Vec<u8>) {
    Mock::given(method("GET"))
        .and(wm_path(path))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(body))
        .mount(server)
        .await;
}

async fn hits(server: &MockServer, path: &str) -> usize {
    server
        .received_requests()
        .await
        .unwrap_or_default()
        .iter()
        .filter(|r| r.url.path() == path)
        .count()
}

fn decode_index(body: &[u8]) -> rattler_conda_types::ShardedRepodata {
    rmp_serde::from_slice(&zstd::decode_all(body).unwrap()).unwrap()
}

fn names(index: &rattler_conda_types::ShardedRepodata) -> Vec<String> {
    let mut names: Vec<String> = index.shards.keys().cloned().collect();
    names.sort();
    names
}

fn decode_shard(body: &[u8]) -> rattler_conda_types::Shard {
    rmp_serde::from_slice(&zstd::decode_all(body).unwrap()).unwrap()
}

/// The streaming parse sees every record of both sections, in document
/// order, and nothing else; trailing garbage is refused.
#[test]
fn records_are_streamed_from_both_sections_4178() {
    let doc = repodata(
        "noarch",
        &["a-1.0-0.tar.bz2", "b-1.0-0.tar.bz2"],
        &["a-2.0-0.conda"],
    );
    let bytes = serde_json::to_vec_pretty(&doc).unwrap();
    let mut seen = Vec::new();
    let n = for_each_record(bytes.as_slice(), |v2, f, r| {
        seen.push((v2, f, r["name"].as_str().unwrap().to_string()));
        Ok(())
    })
    .unwrap();
    assert_eq!(n, 3);
    assert_eq!(
        seen,
        vec![
            (false, "a-1.0-0.tar.bz2".into(), "a".into()),
            (false, "b-1.0-0.tar.bz2".into(), "b".into()),
            (true, "a-2.0-0.conda".into(), "a".into()),
        ]
    );
    let mut bad = bytes.clone();
    bad.extend_from_slice(b" {}");
    assert!(for_each_record(bad.as_slice(), |_, _, _| Ok(())).is_err());
}

/// Spilled and grouped, a name's records from both sections land in one
/// shard; every encoding of the upstream document gives the same shards.
#[test]
fn shards_group_a_name_across_sections_and_encodings_4178() {
    let doc = repodata(
        "noarch",
        &["a-1.0-0.tar.bz2", "b-1.0-0.tar.bz2"],
        &["a-2.0-0.conda"],
    );
    let plain = serde_json::to_vec(&doc).unwrap();
    let zst = zstd::encode_all(plain.as_slice(), 3).unwrap();
    let mut results = Vec::new();
    for (bytes, source) in [
        (plain.clone(), RepodataEncoding::Json),
        (zst, RepodataEncoding::Zst),
        (bz2(&doc), RepodataEncoding::Bz2),
    ] {
        let (parts, records) = spill_document(bytes.into(), None, source).unwrap();
        assert_eq!(records, 3);
        let mut shards = Vec::new();
        for p in &parts.paths {
            shards.extend(shards_of_partition(p).unwrap());
        }
        shards.sort_by(|a, b| a.0.cmp(&b.0));
        results.push(shards);
    }
    assert_eq!(results[0], results[1]);
    assert_eq!(results[0], results[2]);
    let shards = &results[0];
    assert_eq!(
        shards.iter().map(|s| s.0.as_str()).collect::<Vec<_>>(),
        ["a", "b"]
    );
    for (_, bytes, address) in shards {
        assert_eq!(&shard_address(bytes), address);
    }
    let a = decode_shard(&shards[0].1);
    assert_eq!(a.packages.len(), 1);
    assert_eq!(a.conda_packages.len(), 1);
    assert!(a
        .conda_packages
        .keys()
        .any(|k| k.to_string() == "a-2.0-0.conda"));
}

/// #4178 acceptance: a remote whose upstream has no shard index serves a
/// shard index and shards built from the upstream `repodata.json.bz2`, the
/// shards hash to their addresses, and a second request within the cache
/// TTL does not read the upstream document again.
#[tokio::test]
async fn remote_without_upstream_shards_serves_synthesised_ones_4178() {
    let Some(pool) = tdh::try_pool().await else {
        return;
    };
    let server = MockServer::start().await;
    let doc = repodata(
        "noarch",
        &["syn-a-1.0-0.tar.bz2", "syn-b-1.0-0.tar.bz2"],
        &["syn-a-2.0-0.conda"],
    );
    mount(&server, "/noarch/repodata.json.bz2", bz2(&doc)).await;
    let (state, _cache) = state_with_proxy_cache(&pool);
    let (repo_id, repo_key, _dir) = insert_public_remote_conda_repo(&pool, &server.uri()).await;
    let get = |uri: String| {
        let app = tdh::router_anon(super::router(), state.clone());
        async move { tdh::send_with_headers(app, tdh::get(uri)).await }
    };

    let (status, body, _) = get(format!("/{repo_key}/noarch/repodata_shards.msgpack.zst")).await;
    assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
    let index = decode_index(&body);
    assert_eq!(index.info.subdir, "noarch");
    assert_eq!(index.info.shards_base_url, "./shards/");
    assert_eq!(names(&index), ["syn-a", "syn-b"]);
    for (name, hash) in &index.shards {
        let (status, shard, headers) = get(format!(
            "/{repo_key}/noarch/shards/{}.msgpack.zst",
            hex::encode(hash)
        ))
        .await;
        assert_eq!(status, StatusCode::OK, "{name}");
        assert_eq!(shard_address(&shard).as_slice(), hash.as_slice());
        assert_eq!(
            headers.get("cache-control").unwrap(),
            "public, max-age=31536000, immutable"
        );
        let shard = decode_shard(&shard);
        let records = shard.packages.len() + shard.conda_packages.len();
        assert_eq!(records, if name == "syn-a" { 2 } else { 1 });
    }
    let origins: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT origin FROM conda_shard_index WHERE repository_id = $1",
    )
    .bind(repo_id)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(origins, ["synthesized"]);

    let (status, again, _) = get(format!("/{repo_key}/noarch/repodata_shards.msgpack.zst")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(again, body);
    assert_eq!(hits(&server, "/noarch/repodata.json.bz2").await, 1);

    // A stored row whose bytes no longer hash to its address is refused (#4829).
    let hash = &index.shards["syn-a"];
    sqlx::query(
        "UPDATE conda_shard_index SET shard = $3 \
          WHERE repository_id = $1 AND subdir = 'noarch' AND package_name = $2",
    )
    .bind(repo_id)
    .bind("syn-a")
    .bind(b"not the shard".to_vec())
    .execute(&pool)
    .await
    .unwrap();
    let (status, _, _) = get(format!(
        "/{repo_key}/noarch/shards/{}.msgpack.zst",
        hex::encode(hash)
    ))
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    cleanup_conda_repo(&pool, repo_id).await;
}

/// #4178: a package added upstream appears after the refresh, a name the
/// upstream dropped leaves the index, and a refresh that finds the same
/// document changes nothing.
#[tokio::test]
async fn a_changed_upstream_index_is_resynthesised_on_refresh_4178() {
    let Some(pool) = tdh::try_pool().await else {
        return;
    };
    let server = MockServer::start().await;
    let first = repodata("noarch", &[], &["chg-a-1.0-0.conda", "chg-b-1.0-0.conda"]);
    mount(
        &server,
        "/noarch/repodata.json.zst",
        zstd::encode_all(serde_json::to_vec(&first).unwrap().as_slice(), 3).unwrap(),
    )
    .await;
    let (state, _cache) = state_with_proxy_cache(&pool);
    let (repo_id, repo_key, _dir) = insert_public_remote_conda_repo(&pool, &server.uri()).await;
    // Every request revalidates the cached upstream document.
    sqlx::query(
        "INSERT INTO repository_config (repository_id, key, value) VALUES ($1, 'cache_ttl_secs', '0')",
    )
    .bind(repo_id)
    .execute(&pool)
    .await
    .unwrap();
    let index = || {
        let app = tdh::router_anon(super::router(), state.clone());
        let uri = format!("/{repo_key}/noarch/repodata_shards.msgpack.zst");
        async move {
            let (status, body) = tdh::send(app, tdh::get(uri)).await;
            assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
            decode_index(&body)
        }
    };
    let state_row = || async {
        sqlx::query_as::<
            _,
            (
                chrono::DateTime<chrono::Utc>,
                Option<chrono::DateTime<chrono::Utc>>,
            ),
        >(
            "SELECT built_at, stale_since FROM conda_synthesized_subdirs \
              WHERE repository_id = $1 AND subdir = 'noarch'",
        )
        .bind(repo_id)
        .fetch_one(&pool)
        .await
        .unwrap()
    };

    let before = index().await;
    assert_eq!(names(&before), ["chg-a", "chg-b"]);
    let (built, stale) = state_row().await;
    assert!(stale.is_none());

    // Same document again: nothing rebuilt.
    let same = index().await;
    assert_eq!(same.shards, before.shards);
    assert_eq!(state_row().await.0, built);

    server.reset().await;
    let second = repodata("noarch", &[], &["chg-a-1.0-0.conda", "chg-c-1.0-0.conda"]);
    mount(
        &server,
        "/noarch/repodata.json.zst",
        zstd::encode_all(serde_json::to_vec(&second).unwrap().as_slice(), 3).unwrap(),
    )
    .await;
    let after = index().await;
    assert_eq!(names(&after), ["chg-a", "chg-c"]);
    assert_eq!(
        after.shards["chg-a"], before.shards["chg-a"],
        "unchanged shard, same address"
    );
    let (rebuilt, stale) = state_row().await;
    assert!(rebuilt > built && stale.is_none());
    let rows: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM conda_shard_index WHERE repository_id = $1 AND subdir = 'noarch'",
    )
    .bind(repo_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(rows, 2, "the dropped name's shard is gone");
    cleanup_conda_repo(&pool, repo_id).await;
}

/// Peak memory of a synthesis of a real upstream document, without the
/// database: `AK_SYNTH_BENCH_FILE=<repodata.json.zst> cargo test ...
/// synthesis_peak_memory -- --ignored --nocapture`. Prints the record and
/// name counts, the elapsed time and the process's peak RSS (VmHWM) before
/// and after.
#[test]
#[ignore]
fn synthesis_peak_memory() {
    fn hwm() -> String {
        std::fs::read_to_string("/proc/self/status")
            .unwrap_or_default()
            .lines()
            .find(|l| l.starts_with("VmHWM"))
            .unwrap_or("VmHWM: ?")
            .to_string()
    }
    let path = std::env::var("AK_SYNTH_BENCH_FILE").expect("AK_SYNTH_BENCH_FILE");
    let source = if path.ends_with(".zst") {
        RepodataEncoding::Zst
    } else if path.ends_with(".bz2") {
        RepodataEncoding::Bz2
    } else {
        RepodataEncoding::Json
    };
    let bytes = std::fs::read(&path).unwrap();
    println!("input {} bytes compressed; before: {}", bytes.len(), hwm());
    let started = std::time::Instant::now();
    let (parts, records) = spill_document(bytes.into(), None, source).unwrap();
    let spilled = started.elapsed();
    let mut names = 0usize;
    let mut shard_bytes = 0usize;
    for p in &parts.paths {
        let shards = shards_of_partition(p).unwrap();
        names += shards.len();
        shard_bytes += shards.iter().map(|s| s.1.len()).sum::<usize>();
    }
    println!(
        "records {records}, names {names}, shard bytes {shard_bytes}, spill {:?}, total {:?}; after: {}",
        spilled,
        started.elapsed(),
        hwm()
    );
    let _ = shard_synth::PARTITIONS;
}
