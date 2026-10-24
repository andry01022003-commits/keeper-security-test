//! Router-level tests of the hosted JLAP (#4175). They use only the router
//! and SQL, so they also run against a build that serves the bootstrap JLAP
//! (where they fail).

use super::revision_router_tests::{get, upload};
use crate::api::handlers::test_db_helpers as tdh;
use axum::http::StatusCode;
use serde_json::Value;

fn blake2b_256(data: &[u8]) -> String {
    use blake2::digest::consts::U32;
    use blake2::digest::FixedOutput;
    let mut h = blake2::Blake2b::<U32>::default();
    blake2::digest::Update::update(&mut h, data);
    hex::encode(h.finalize_fixed())
}

fn keyed(data: &[u8], key: &[u8]) -> Vec<u8> {
    use blake2::digest::consts::U32;
    use blake2::digest::{FixedOutput, KeyInit};
    let mut h = blake2::Blake2bMac::<U32>::new_from_slice(key).unwrap();
    blake2::digest::Update::update(&mut h, data);
    h.finalize_fixed().to_vec()
}

/// A JLAP read the way conda's reader reads it: `(offset, line, chain)` per
/// line, the chain started from `iv` unless the read starts at offset 0
/// (then the first line is the IV). Panics when the trailing checksum does
/// not match the chain.
fn read_jlap(bytes: &[u8], pos: usize, iv: &[u8]) -> Vec<(usize, String, Vec<u8>)> {
    let mut out = Vec::new();
    let mut chain = iv.to_vec();
    let mut at = pos;
    for line in bytes.split(|b| *b == b'\n') {
        if at == 0 {
            chain = hex::decode(line).expect("IV line is hex");
        } else {
            chain = keyed(line, &chain);
        }
        out.push((at, String::from_utf8(line.to_vec()).unwrap(), chain.clone()));
        at += line.len() + 1;
    }
    let n = out.len();
    assert!(n >= 2, "a JLAP has at least a footer and a checksum");
    assert_eq!(
        out[n - 1].1,
        hex::encode(&out[n - 2].2),
        "the trailing checksum continues the chain"
    );
    out
}

/// The patch lines and the footer's `latest` of a read JLAP.
fn patches_and_latest(read: &[(usize, String, Vec<u8>)], from_zero: bool) -> (Vec<Value>, String) {
    let n = read.len();
    let body = &read[usize::from(from_zero)..n - 2];
    let patches = body
        .iter()
        .map(|(_, l, _)| serde_json::from_str(l).expect("patch lines are JSON"))
        .collect();
    let footer: Value = serde_json::from_str(&read[n - 2].1).unwrap();
    (patches, footer["latest"].as_str().unwrap().to_string())
}

/// RFC 6902 `add`, `remove`, `replace` on object members, as a JSON Patch
/// library applies them.
fn apply(doc: &mut Value, ops: &Value) {
    for op in ops.as_array().unwrap() {
        let path = op["path"].as_str().unwrap();
        let tokens: Vec<String> = path
            .split('/')
            .skip(1)
            .map(|t| t.replace("~1", "/").replace("~0", "~"))
            .collect();
        let (last, parents) = tokens.split_last().unwrap();
        let mut target = &mut *doc;
        for t in parents {
            target = target.get_mut(t.as_str()).expect("parent exists");
        }
        let map = target.as_object_mut().unwrap();
        match op["op"].as_str().unwrap() {
            "add" => {
                map.insert(last.clone(), op["value"].clone());
            }
            "replace" => {
                assert!(map.contains_key(last), "replace of a missing {path}");
                map.insert(last.clone(), op["value"].clone());
            }
            "remove" => {
                assert!(map.remove(last).is_some(), "remove of a missing {path}");
            }
            other => panic!("unexpected op {other}"),
        }
    }
}

/// What conda's `find_patches` does: walk back from `want` to `have`.
fn patches_from(patches: &[Value], have: &str, want: &str) -> Option<Vec<Value>> {
    let mut apply = Vec::new();
    let mut want = want.to_string();
    for p in patches.iter().rev() {
        if want == have {
            break;
        }
        if p["to"] == want.as_str() {
            apply.push(p["patch"].clone());
            want = p["from"].as_str().unwrap().to_string();
        }
    }
    (want == have).then(|| apply.into_iter().rev().collect())
}

async fn send_range(
    fx: &tdh::Fixture,
    suffix: &str,
    range: &str,
) -> (StatusCode, bytes::Bytes, axum::http::HeaderMap) {
    let req = axum::http::Request::builder()
        .method("GET")
        .uri(format!("/{}/{suffix}", fx.repo_key))
        .header("Range", range)
        .body(axum::body::Body::empty())
        .unwrap();
    tdh::send_with_headers(fx.router_with_auth(super::router()), req).await
}

/// #4175 acceptance: a client holding `repodata.json` at one revision
/// updates it from the JLAP alone, to a document equal to a full fetch; the
/// JLAP verifies, and every encoding hashes to what its footer names.
#[tokio::test]
async fn jlap_patches_a_cached_index_to_the_current_one_4175() {
    let Some(fx) = tdh::Fixture::setup("local", "conda").await else {
        return;
    };
    upload(&fx, "noarch", "jl-a", "1.0").await;
    let (status, r1, _) = get(&fx, "noarch/repodata.json").await;
    assert_eq!(status, StatusCode::OK);
    let have = blake2b_256(&r1);

    let added = upload(&fx, "noarch", "jl-b", "1.0").await;
    let (status, jlap, headers) = get(&fx, "noarch/repodata.jlap").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers.get("accept-ranges").unwrap(), "bytes");
    let read = read_jlap(&jlap, 0, &[]);
    let (patches, want) = patches_and_latest(&read, true);

    let (_, r2, _) = get(&fx, "noarch/repodata.json").await;
    assert_eq!(
        want,
        blake2b_256(&r2),
        "the footer names the served document"
    );
    let (_, zst, _) = get(&fx, "noarch/repodata.json.zst").await;
    assert_eq!(
        blake2b_256(&zstd::stream::decode_all(zst.as_ref()).unwrap()),
        want,
        "repodata.json.zst holds the bytes the chain hashes"
    );

    let apply_ops = patches_from(&patches, &have, &want)
        .expect("the JLAP leads from the cached revision to the current one");
    assert_eq!(apply_ops.len(), 1);
    let mut doc: Value = serde_json::from_slice(&r1).unwrap();
    for ops in &apply_ops {
        apply(&mut doc, ops);
    }
    assert_eq!(doc, serde_json::from_slice::<Value>(&r2).unwrap());
    assert!(doc["packages"].get(&added).is_some());
}

/// #4175: a withdrawal is absent from the JLAP tail as soon as it is absent
/// from `repodata.json`, and the range request a client makes from the
/// footer it saw returns only the new lines, continuing its chain. The
/// withdrawal is a plain catalog update, and the JLAP is requested before
/// `repodata.json` is read again.
#[tokio::test]
async fn a_withdrawal_reaches_the_jlap_tail_by_range_request_4175() {
    let Some(fx) = tdh::Fixture::setup("local", "conda").await else {
        return;
    };
    upload(&fx, "noarch", "jw-a", "1.0").await;
    let pulled = upload(&fx, "noarch", "jw-b", "1.0").await;
    let (_, r1, _) = get(&fx, "noarch/repodata.json").await;
    assert!(String::from_utf8_lossy(&r1).contains(&pulled));
    let (_, jlap1, _) = get(&fx, "noarch/repodata.jlap").await;
    let read1 = read_jlap(&jlap1, 0, &[]);
    let n = read1.len();
    // What conda saves: the footer's offset and the chain before it.
    let (pos, iv) = (read1[n - 2].0, read1[n - 3].2.clone());

    sqlx::query(
        "UPDATE artifacts SET quarantine_status = 'rejected' \
          WHERE repository_id = $1 AND path = $2",
    )
    .bind(fx.repo_id)
    .bind(format!("noarch/{pulled}"))
    .execute(&fx.pool)
    .await
    .unwrap();

    let (status, tail, headers) =
        send_range(&fx, "noarch/repodata.jlap", &format!("bytes={pos}-")).await;
    assert_eq!(status, StatusCode::PARTIAL_CONTENT);
    let total: usize = headers
        .get("content-range")
        .unwrap()
        .to_str()
        .unwrap()
        .rsplit('/')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        tail.len(),
        total - pos,
        "only the bytes from the saved footer"
    );
    let read2 = read_jlap(&tail, pos, &iv);
    let (patches, want) = patches_and_latest(&read2, false);
    let removes = patches
        .iter()
        .flat_map(|p| p["patch"].as_array().unwrap().clone())
        .filter(|op| op["op"] == "remove")
        .map(|op| op["path"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(removes, vec![format!("/packages/{pulled}")]);

    let (_, r2, _) = get(&fx, "noarch/repodata.json").await;
    assert!(!String::from_utf8_lossy(&r2).contains(&format!("\"{pulled}\": {{")));
    assert_eq!(want, blake2b_256(&r2));
    let mut doc: Value = serde_json::from_slice(&r1).unwrap();
    for ops in patches_from(&patches, &blake2b_256(&r1), &want).unwrap() {
        apply(&mut doc, &ops);
    }
    assert_eq!(doc, serde_json::from_slice::<Value>(&r2).unwrap());

    // A saved position past the end (a client of a trimmed file) is 416, so
    // the client fetches the whole file.
    let (status, _, _) = send_range(
        &fx,
        "noarch/repodata.json.jlap",
        &format!("bytes={}-", total + 10),
    )
    .await;
    assert_eq!(status, StatusCode::RANGE_NOT_SATISFIABLE);
}
