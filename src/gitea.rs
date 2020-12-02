use serde::{Deserialize, Serialize};

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
pub struct Response {
    pub r#ref: String,
    pub before: String,      // commit hash
    pub after: String,       // commit hash
    pub compare_url: String, // url with diff?
    pub commits: Vec<Commit>,
    pub repository: Repository,
    pub pusher: GiteaUser,
    pub sender: GiteaUser,
}
