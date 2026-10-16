//! #4642 regression gate: the artifact upload metrics must keep a production
//! producer.
//!
//! `metrics_service::record_artifact_upload` populates
//! `ak_artifact_uploads_total` and the `ak_artifact_upload_size_bytes`
//! histogram. Both were defined but called from nowhere except
//! `metrics_service`'s own unit tests, so a `/metrics` scrape carried the
//! `ak_http_*` series and none of the artifact ones, leaving dashboards that
//! query them permanently empty.
//!
//! The call now lives at the shared service-layer upload choke point
//! (`ArtifactService::finalize_upload`). This is a source-level ratchet in the
//! spirit of `streaming_invariant.rs`: it fails if that call site is deleted,
//! which is exactly how the metric lost its producer in the first place. It
//! deliberately asserts the *existence of a caller* rather than a rendered
//! sample, because reaching `finalize_upload` requires a live database and the
//! regression being guarded is a missing call, not a broken counter.

use std::path::{Path, PathBuf};

/// The module that defines the helper, plus its own `#[cfg(test)]` callers.
/// Excluded so the gate measures real producers only.
const DEFINING_MODULE: &str = "services/metrics_service.rs";

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in rd.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn record_artifact_upload_has_a_production_caller() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src = root.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src, &mut files);
    files.sort();

    let mut callers: Vec<String> = Vec::new();
    for file in &files {
        let rel = file
            .strip_prefix(&src)
            .unwrap_or(file)
            .to_string_lossy()
            .replace('\\', "/");
        if rel == DEFINING_MODULE {
            continue;
        }
        let raw = std::fs::read_to_string(file).expect("read source file");
        if raw.contains("record_artifact_upload(") {
            callers.push(rel);
        }
    }

    assert!(
        !callers.is_empty(),
        "ak_artifact_uploads_total / ak_artifact_upload_size_bytes have no \
         producer outside {DEFINING_MODULE}: nothing in the production tree \
         calls record_artifact_upload, so a /metrics scrape exposes neither \
         series (#4642)"
    );
}
