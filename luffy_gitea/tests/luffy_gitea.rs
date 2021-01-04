use luffy_gitea::payloads::*;
use std::fs;
use std::path::PathBuf;

// Simple tests to ensure serde definitions are correct.
// Should assert values as well.
#[test]
fn deserialize_push_payload() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/resources/gitea/push_payload.json");
    let data = fs::read_to_string(d).expect("Unable to read file");
    serde_json::from_str::<PushPayload>(&data).expect("Deserialization error");
}

#[test]
fn deserialize_pull_request_payload() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/resources/gitea/pull_request_payload.json");
    let data = fs::read_to_string(d).expect("Unable to read file");
    serde_json::from_str::<PullRequestPayload>(&data).expect("Deserialization error");
}

#[test]
fn deserialize_issue_comment_payload() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/resources/gitea/issue_comment_payload.json");
    let data = fs::read_to_string(d).expect("Unable to read file");
    serde_json::from_str::<IssueCommentPayload>(&data).expect("Deserialization error");
}

#[test]
fn deserialize_issues_payload() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/resources/gitea/issues_payload.json");
    let data = fs::read_to_string(d).expect("Unable to read file");
    serde_json::from_str::<IssueCommentPayload>(&data).expect("Deserialization error");
}
