//! `GET/PUT/DELETE /api/v1/repositories/{key}/proxy-cache-cap` (#4660): the
//! byte cap on a Remote repository's proxy cache, and its current usage.
//!
//! Mounted on the `/repositories` nest from `routes.rs`; the enforcement lives
//! in [`crate::services::proxy_cache_cap`].

use axum::{
    extract::{Path, State},
    routing::get,
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::api::handlers::repositories::{require_repo_admin, require_repo_write_access};
use crate::api::middleware::auth::AuthExtension;
use crate::api::SharedState;
use crate::error::{AppError, Result};
use crate::models::repository::{Repository, RepositoryType};
use crate::services::proxy_cache_cap::{self, CacheUsage, AT_CAP_BEHAVIOUR};
use crate::services::repository_service::RepositoryService;

pub fn repo_router() -> Router<SharedState> {
    Router::new().route(
        "/:key/proxy-cache-cap",
        get(get_proxy_cache_cap)
            .put(set_proxy_cache_cap)
            .delete(delete_proxy_cache_cap),
    )
}

/// Body of `PUT .../proxy-cache-cap`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct SetProxyCacheCapRequest {
    /// The most bytes the repository's proxy cache may hold. Must be positive;
    /// remove the cap with `DELETE`.
    pub cap_bytes: i64,
}

/// A Remote repository's proxy-cache cap and current usage.
#[derive(Debug, Serialize, ToSchema)]
pub struct ProxyCacheCapResponse {
    pub repository_key: String,
    /// The cap in bytes, or `null` when the cache is uncapped.
    pub cap_bytes: Option<i64>,
    /// What happens when a fill takes the cache over the cap: `evict_lru`
    /// (least recently used entries are evicted until usage is back under the
    /// cap; new fills are refused only while eviction cannot get there).
    pub at_cap: String,
    /// What the cache holds now.
    pub usage: CacheUsage,
    /// The most recent eviction left the cache over its cap, so new fills are
    /// served without being cached (this replica's view).
    pub refusing_fills: bool,
}

/// Resolve `key` to a Remote repository the caller administers. The cap
/// decides what the cache keeps, the same tier as the upstream filter and the
/// cache TTL, so reading and changing it both need the repository `admin`
/// action.
async fn cap_repo(
    state: &SharedState,
    auth: Option<AuthExtension>,
    key: &str,
    write: bool,
) -> Result<Repository> {
    let auth = auth.ok_or_else(|| AppError::Authentication("Authentication required".into()))?;
    auth.require_scope(if write {
        "write:repositories"
    } else {
        "read:repositories"
    })?;
    let service = RepositoryService::new(state.db.clone());
    let repo = service.get_by_key(key).await?;
    if !auth.can_access_repo(repo.id) {
        return Err(AppError::Authorization(
            "Token does not have access to this repository".to_string(),
        ));
    }
    if repo.repo_type != RepositoryType::Remote {
        return Err(AppError::Validation(
            "A proxy-cache cap applies to remote repositories only".to_string(),
        ));
    }
    if write {
        require_repo_write_access(&auth, &repo, &service).await?;
    }
    require_repo_admin(&auth, repo.id, &state.permission_service).await?;
    Ok(repo)
}

async fn response(state: &SharedState, repo: &Repository) -> Result<ProxyCacheCapResponse> {
    let cap_bytes = proxy_cache_cap::load_cap(&state.db, repo.id).await?;
    let usage = proxy_cache_cap::usage(&state.db, repo.id).await?;
    let refusing_fills = match state.proxy_service.as_ref() {
        Some(proxy) => !proxy.cache_cap().admits_fills(repo.id).await,
        None => false,
    };
    Ok(ProxyCacheCapResponse {
        repository_key: repo.key.clone(),
        cap_bytes,
        at_cap: AT_CAP_BEHAVIOUR.to_string(),
        usage,
        refusing_fills,
    })
}

/// Get the proxy-cache cap and usage of a remote repository
#[utoipa::path(
    get,
    path = "/{key}/proxy-cache-cap",
    context_path = "/api/v1/repositories",
    tag = "repositories",
    params(("key" = String, Path, description = "Repository key")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Cap and usage", body = ProxyCacheCapResponse),
        (status = 400, description = "Repository is not a remote repository"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Repository admin required"),
        (status = 404, description = "Repository not found"),
    )
)]
pub async fn get_proxy_cache_cap(
    State(state): State<SharedState>,
    Extension(auth): Extension<Option<AuthExtension>>,
    Path(key): Path<String>,
) -> Result<Json<ProxyCacheCapResponse>> {
    let repo = cap_repo(&state, auth, &key, false).await?;
    Ok(Json(response(&state, &repo).await?))
}

/// Set the proxy-cache cap of a remote repository
///
/// Caps the bytes the repository's proxy cache holds. After every cache fill
/// the least recently used entries (by last serve or last write) are evicted
/// until usage is at or under the cap, so usage can exceed it briefly by the
/// fill that just committed. An object larger than the cap is served without
/// being cached. If eviction cannot get under the cap (storage refuses the
/// deletes), new fills are served without being cached until it can. Setting
/// a cap below current usage evicts at once. Evicted entries keep their
/// download history.
#[utoipa::path(
    put,
    path = "/{key}/proxy-cache-cap",
    context_path = "/api/v1/repositories",
    tag = "repositories",
    params(("key" = String, Path, description = "Repository key")),
    request_body = SetProxyCacheCapRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Cap saved; usage after the eviction it triggered", body = ProxyCacheCapResponse),
        (status = 400, description = "cap_bytes is not positive, or not a remote repository"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Repository admin required"),
        (status = 404, description = "Repository not found"),
    )
)]
pub async fn set_proxy_cache_cap(
    State(state): State<SharedState>,
    Extension(auth): Extension<Option<AuthExtension>>,
    Path(key): Path<String>,
    Json(payload): Json<SetProxyCacheCapRequest>,
) -> Result<Json<ProxyCacheCapResponse>> {
    let repo = cap_repo(&state, auth, &key, true).await?;
    proxy_cache_cap::save_cap(&state.db, repo.id, Some(payload.cap_bytes)).await?;
    if let Some(proxy) = state.proxy_service.as_ref() {
        let cap = proxy.cache_cap();
        cap.invalidate(repo.id).await;
        // Apply a cap below current usage now rather than at the next fill.
        cap.enforce(repo.id, payload.cap_bytes).await?;
    }
    Ok(Json(response(&state, &repo).await?))
}

/// Remove the proxy-cache cap of a remote repository
#[utoipa::path(
    delete,
    path = "/{key}/proxy-cache-cap",
    context_path = "/api/v1/repositories",
    tag = "repositories",
    params(("key" = String, Path, description = "Repository key")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Cap removed", body = ProxyCacheCapResponse),
        (status = 400, description = "Repository is not a remote repository"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Repository admin required"),
        (status = 404, description = "Repository not found"),
    )
)]
pub async fn delete_proxy_cache_cap(
    State(state): State<SharedState>,
    Extension(auth): Extension<Option<AuthExtension>>,
    Path(key): Path<String>,
) -> Result<Json<ProxyCacheCapResponse>> {
    let repo = cap_repo(&state, auth, &key, true).await?;
    proxy_cache_cap::save_cap(&state.db, repo.id, None).await?;
    if let Some(proxy) = state.proxy_service.as_ref() {
        proxy.cache_cap().invalidate(repo.id).await;
    }
    Ok(Json(response(&state, &repo).await?))
}

#[derive(OpenApi)]
#[openapi(
    paths(get_proxy_cache_cap, set_proxy_cache_cap, delete_proxy_cache_cap),
    components(schemas(SetProxyCacheCapRequest, ProxyCacheCapResponse, CacheUsage))
)]
pub struct ProxyCacheCapApiDoc;
