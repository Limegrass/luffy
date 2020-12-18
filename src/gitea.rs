use crate::core::Service;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

// use chrono::DateTime if I'm doing more than just forwarding
type DateTimeType = String; // "2017-03-13T13:52:11-04:00"
type GiteaIntId = u32; // not sure u32 vs u64

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GiteaUser {
    pub id: GiteaIntId,
    pub login: String,
    pub full_name: String,
    pub email: String,
    pub avatar_url: String,
    pub username: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Commit {
    pub id: String, // uuid
    pub message: String,
    pub url: String, // url with scheme
    pub author: GitUser,
    pub committer: GitUser,
    pub timestamp: DateTimeType,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Repository {
    pub id: GiteaIntId,
    pub owner: GiteaUser,
    pub name: String,
    pub full_name: String, // GiteaUsername/RepositoryName
    pub description: String,
    pub private: bool,
    pub fork: bool,
    pub html_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub website: String,
    pub stars_count: u32,
    pub forks_count: u32,
    pub watchers_count: u32,
    pub open_issues_count: u32,
    pub default_branch: String,
    pub created_at: DateTimeType,
    pub updated_at: DateTimeType,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub ref_path: String,
    pub before: String,      // commit hash
    pub after: String,       // commit hash
    pub compare_url: String, // url with diff?
    pub commits: Vec<Commit>,
    pub repository: Repository,
    pub pusher: GiteaUser,
    pub sender: GiteaUser,
}

impl TryFrom<&str> for PushEvent {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s).map_err(|e| format!("{:?}", e))
    }
}

// Support only Gitea for the time being.
// Would have to reconcile differences in event payloads otherwise
pub const PUSH_EVENT_HEADER_NAME: &'static str = "X-Gitea-Event";

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct ForkEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct IssuesEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct IssueAssignEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct IssueLabelEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct IssueMilestoneEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct IssueCommentEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestAssignEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestLabelEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestMilestoneEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestCommentEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestReviewEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestSyncEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryEvent;
#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseEvent;

#[derive(Debug, Deserialize, Serialize)]
pub enum HookEvent {
    Create(CreateEvent),
    Delete(DeleteEvent),
    Fork(ForkEvent),
    Issues(IssuesEvent),
    IssueAssign(IssueAssignEvent),
    IssueLabel(IssueLabelEvent),
    IssueMilestone(IssueMilestoneEvent),
    IssueComment(IssueCommentEvent),
    Push(PushEvent),
    PullRequest(PullRequestEvent),
    PullRequestAssign(PullRequestAssignEvent),
    PullRequestLabel(PullRequestLabelEvent),
    PullRequestMilestone(PullRequestMilestoneEvent),
    PullRequestComment(PullRequestCommentEvent),
    PullRequestReview(PullRequestReviewEvent),
    PullRequestSync(PullRequestSyncEvent),
    Repository(RepositoryEvent),
    Release(ReleaseEvent),
}

pub struct Gitea;

impl Service<HookEvent, String> for Gitea {
    fn event_header_name(&self) -> &'static str {
        "X-Gitea-Event"
    }

    fn parse_hook_event(
        &self,
        hook_event_type: &str,
        hook_event_body: &str,
    ) -> Result<HookEvent, String> {
        match hook_event_type {
            "create" => Ok(HookEvent::Create(CreateEvent)),
            "delete" => Ok(HookEvent::Delete(DeleteEvent)),
            "fork" => Ok(HookEvent::Fork(ForkEvent)),
            "issues" => Ok(HookEvent::Issues(IssuesEvent)),
            "issue_assign" => Ok(HookEvent::IssueAssign(IssueAssignEvent)),
            "issue_label" => Ok(HookEvent::IssueLabel(IssueLabelEvent)),
            "issue_milestone" => Ok(HookEvent::IssueMilestone(IssueMilestoneEvent)),
            "issue_comment" => Ok(HookEvent::IssueComment(IssueCommentEvent)),
            "push" => Ok(HookEvent::Push(hook_event_body.try_into()?)),
            "pull_request" => Ok(HookEvent::PullRequest(PullRequestEvent)),
            "pull_request_assign" => Ok(HookEvent::PullRequestAssign(PullRequestAssignEvent)),
            "pull_request_label" => Ok(HookEvent::PullRequestLabel(PullRequestLabelEvent)),
            "pull_request_milestone" => {
                Ok(HookEvent::PullRequestMilestone(PullRequestMilestoneEvent))
            }
            "pull_request_comment" => Ok(HookEvent::PullRequestComment(PullRequestCommentEvent)),
            "pull_request_review" => Ok(HookEvent::PullRequestReview(PullRequestReviewEvent)),
            "pull_request_sync" => Ok(HookEvent::PullRequestSync(PullRequestSyncEvent)),
            "repository" => Ok(HookEvent::Repository(RepositoryEvent)),
            "release" => Ok(HookEvent::Release(ReleaseEvent)),
            _ => Err(String::from("unrecognized git event")),
        }
    }
}
