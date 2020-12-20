// https://github.com/go-gitea/gitea/blob/master/modules/notification/webhook/webhook.go
use crate::structs::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// TODO: Deserves proper Error enum later.
pub type JsonParseError = String;

// TODO: Change to a proper derive macro
// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
macro_rules! impl_serde_deserialize {
    ($type: ty) => {
        impl TryFrom<&str> for $type {
            type Error = JsonParseError;
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                serde_json::from_str(s).map_err(|e| format!("{:?}", e))
            }
        }
    };
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePayload {
    pub action: String, // HookRepoAction: "created" or "deleted"
    pub repository: Repository,
    pub organization: Organization,
    pub sender: GiteaUser,
}
impl_serde_deserialize!(CreatePayload);

// They're the same fields. Redefine?
pub type DeletePayload = CreatePayload;

#[derive(Debug, Deserialize, Serialize)]
pub struct ForkPayload {
    pub forkee: Repository,
    pub repo: Repository,
    pub sender: GiteaUser,
}
impl_serde_deserialize!(ForkPayload);

#[derive(Debug, Deserialize, Serialize)]
pub struct IssuePayload {
    pub number: i64,
    pub action: String, // HookIssueAction: "opened", ...
    pub changes: Option<Changes>,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: GiteaUser,
}
impl_serde_deserialize!(IssuePayload);

#[derive(Debug, Deserialize, Serialize)]
pub struct IssueCommentPayload {
    pub action: String, // HookIssueCommentAction: "created", ...
    pub issue: Issue,
    pub comment: Comment,
    pub changes: Option<Changes>,
    pub repository: Repository,
    pub sender: GiteaUser,
    pub is_pull: bool,
}
impl_serde_deserialize!(IssueCommentPayload);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PushPayload {
    #[serde(rename = "ref")]
    pub ref_path: String,
    pub before: String,      // commit hash
    pub after: String,       // commit hash
    pub compare_url: String, // url with diff?
    pub commits: Vec<Commit>,
    pub head_commit: Commit,
    pub repository: Repository,
    pub pusher: GiteaUser,
    pub sender: GiteaUser,
}
impl_serde_deserialize!(PushPayload);

#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestPayload {
    pub action: String, // HookIssueAction
    pub number: i64,
    pub changes: Option<Changes>,
    // pub pull_request *PullRequest    `json:"pull_request"`
    pub repository: Repository,
    pub sender: GiteaUser,
    // pub review : ReviewPayload // looks to be wip
}
impl_serde_deserialize!(PullRequestPayload);

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryPayload {
    pub action: String, // HookRepoAction
    pub repository: Repository,
    pub organization: Organization,
    pub sender: GiteaUser,
}
impl_serde_deserialize!(RepositoryPayload);

#[derive(Debug, Deserialize, Serialize)]
pub struct ReleasePayload {
    pub action: String, // HookReleaseAction
    pub release: Release,
    pub repository: Repository,
    pub sender: GiteaUser,
}
impl_serde_deserialize!(ReleasePayload);
