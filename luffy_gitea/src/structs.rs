use serde::{Deserialize, Serialize};

// use chrono::DateTime if I'm doing more than just forwarding
type DateTimeType = String; // "2017-03-13T13:52:11-04:00"

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GiteaUser {
    pub id: i64,
    pub login: String,
    pub full_name: String,
    pub email: String,
    pub avatar_url: String,
    pub username: String,
    pub language: String,
    pub is_admin: bool,
    pub last_login: DateTimeType,
    pub created: DateTimeType,
}

// They're the same type in Gitea Might make sense to redefine
// or to just reference GiteaUser in usages later
pub type Organization = GiteaUser;

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
pub struct Milestone {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub state: String, // StateType: "open", "closed", "all"
    pub open_issues: i32,
    pub closed_issues: i32,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub due_on: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Repository {
    pub id: i64,
    pub owner: GiteaUser,
    pub name: String,
    pub full_name: String, // GiteaUsername/RepositoryName
    pub description: String,
    #[serde(rename = "empty")]
    pub is_empty: bool,
    #[serde(rename = "private")]
    pub is_private: bool,
    #[serde(rename = "fork")]
    pub is_fork: bool,
    #[serde(rename = "template")]
    pub is_template: bool,
    pub parent: Option<Box<Repository>>,
    #[serde(rename = "mirror")]
    pub is_mirror: bool,
    #[serde(rename = "size")]
    pub size_mib: i32,
    pub html_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub original_url: String,
    pub website: String,
    pub stars_count: i32,
    pub forks_count: i32,
    pub watchers_count: i32,
    pub open_issues_count: i32,
    pub open_pr_counter: i32,
    pub release_counter: i32,
    pub default_branch: String,
    #[serde(rename = "archived")]
    pub is_archived: bool,
    pub created_at: DateTimeType,
    pub updated_at: DateTimeType,
    #[serde(default)]
    pub permissions: Option<Permissions>,
    pub has_issues: bool,
    #[serde(default)]
    pub internal_tracker: Option<InternalTracker>,
    #[serde(default)]
    pub external_tracker: Option<ExternalTracker>,
    pub has_wiki: bool,
    #[serde(default)]
    pub external_wiki: Option<ExternalWiki>,
    pub has_pull_requests: bool,
    pub has_projects: bool,
    #[serde(rename = "ignore_whitespace_conflicts")]
    pub is_whitespace_conflict_ignored: bool,
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_squash_merge: bool,
    pub avatar_url: String,
    #[serde(rename = "internal")]
    pub is_internal: bool,
}

// yuck but idc rn
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct InternalTracker {
    #[serde(rename = "enable_time_tracker")]
    pub is_enabled: bool,
    #[serde(rename = "allow_only_contributors_to_track_time")]
    pub is_contributor_only_time_tracking_enabled: bool,
    #[serde(rename = "enable_issue_dependencies")]
    pub is_issue_dependencies_enabled: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ExternalTracker {
    pub external_tracker_url: String,
    // External Issue Tracker URL Format.
    // Use the placeholders {user},
    // {repo} and {index} for the username,
    // repository name and issue index.
    pub external_tracker_format: String,
    pub external_tracker_style: String, // `numeric` or `alphanumeric`
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ExternalWiki {
    #[serde(rename = "external_wiki_url")]
    pub url: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Permissions {
    #[serde(rename = "admin")]
    pub has_admin: bool,
    #[serde(rename = "push")]
    pub has_push: bool,
    #[serde(rename = "pull")]
    pub has_pull: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Issue {
    pub id: i64,
    #[serde(rename = "url")]
    pub api_url: String,
    pub html_url: String,
    pub number: i64,
    pub user: GiteaUser,
    pub original_author: String,
    pub original_author_id: i64,
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
    pub milestone: Option<Milestone>,
    pub assignee: Option<GiteaUser>,
    pub assignees: Option<Vec<GiteaUser>>,
    pub state: String, // "open", "closed", "all"
    pub is_locked: bool,
    #[serde(rename = "comments")]
    pub comment_count: i32,
    pub created_at: DateTimeType,
    pub updated_at: DateTimeType, // updated = created on open?
    pub closed_at: Option<DateTimeType>,
    pub due_date: Option<DateTimeType>,
    #[serde(rename = "pull_request")]
    pub pull_request_meta: Option<PullRequestMeta>,
    #[serde(rename = "repository")]
    pub repository_meta: Option<RepositoryMeta>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PullRequestMeta {
    #[serde(rename = "merged")]
    pub is_merged: bool,
    #[serde(rename = "merged_at")]
    pub time_merged: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RepositoryMeta {
    pub id: i64,
    pub name: String,
    pub owner: String,
    pub full_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub id: i64,
    pub html_url: String,
    pub pull_request_url: String,
    pub issue_url: String,
    pub user: GiteaUser,
    pub original_author: String,
    pub original_author_id: i64,
    #[serde(rename = "body")]
    pub comment_text: String,
    pub created_at: DateTimeType,
    pub updated_at: DateTimeType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Changes {
    #[serde(default)]
    pub title: Option<ChangesFromPayload>,
    #[serde(default)]
    pub body: Option<ChangesFromPayload>,
    #[serde(default, rename = "ref")]
    pub ref_path: Option<ChangesFromPayload>,
}

// TODO: Maybe get rid of this mirroring
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangesFromPayload {
    pub from: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    pub id: i64,
    pub tag_name: String,
    pub target_commitish: String, // branch?
    pub name: String,             // title
    pub body: String,             // note
    #[serde(rename = "url")]
    pub api_url: String,
    pub html_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    #[serde(rename = "draft")]
    pub is_draft: bool,
    #[serde(rename = "prelease")]
    pub is_prerelease: bool,
    pub created_at: DateTimeType,
    pub published_at: DateTimeType,
    pub author: GiteaUser,
    pub assets: Vec<Attachment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment {
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: DateTimeType,
    pub uuid: String,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequest {
    pub id: i64,
    pub url: String,
    pub number: i64,
    pub user: GiteaUser,
    pub title: String,
    pub body: String,
    // TODO: labels: Vec<Label>
    pub milestone: Option<Milestone>,
    pub assignee: Option<GiteaUser>,
    pub assignees: Option<Vec<GiteaUser>>,
    pub state: String, // StateType
    pub is_locked: bool,
    #[serde(rename = "comments")]
    pub comment_count: i32,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    #[serde(rename = "mergeable")]
    pub is_mergeable: bool,
    #[serde(rename = "merged")]
    pub is_merged: bool,
    pub merged_at: Option<DateTimeType>,
    pub merge_commit_sha: Option<String>,
    pub merged_by: Option<GiteaUser>,
    pub base: Option<PRBranchInfo>,
    pub head: Option<PRBranchInfo>,
    pub merge_base: String,
    pub due_date: Option<DateTimeType>,
    pub created_at: Option<DateTimeType>,
    pub updated_at: Option<DateTimeType>,
    pub closed_at: Option<DateTimeType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PRBranchInfo {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_path: String,
    pub sha: String,
    pub repo_id: i64,
    #[serde(rename = "repo")]
    pub repository: Repository,
}
