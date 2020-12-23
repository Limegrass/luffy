use async_trait::async_trait;
use log::*;
use luffy_core::Handler;
use luffy_gitea::{payloads::*, structs::*, HookEvent};
use serde::Deserialize;
use std::fs::read_to_string;
use std::process::Command;

// TODO: allow yaml, toml
// TODO: allow inlined commands in cfg
pub struct GiteaCliHandler {
    config_path: String,
}

impl GiteaCliHandler {
    pub fn new(config_path: &str) -> GiteaCliHandler {
        GiteaCliHandler {
            config_path: config_path.to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    create: String,
    delete: String,
    fork: String,
    issues: String,
    issue_comment: String,
    push: String,
    pull_request: String,
    repository: String,
    release: String,
}

// TODO: Make the custom derive, if it's possible
// (but it's fucking impossible)
#[async_trait]
impl Handler<HookEvent> for GiteaCliHandler {
    async fn handle_event(&self, event: &HookEvent) {
        // TODO: Maybe get rid of the event type prefixing in the "handle"
        let config_string = read_to_string(&self.config_path).expect("TODO: return Err");
        let config = serde_json::from_str(&config_string).expect("but really though");

        let mut command = match event {
            HookEvent::Create(payload) => get_create_command(&config, payload),
            HookEvent::Delete(payload) => get_delete_command(&config, payload),
            HookEvent::Fork(payload) => get_fork_command(&config, payload),
            HookEvent::Issues(payload) => get_issue_command(&config, payload),
            HookEvent::IssueComment(payload) => get_issue_comment_command(&config, payload),
            HookEvent::PullRequest(payload) => get_pull_request_command(&config, payload),
            HookEvent::Push(payload) => get_push_command(&config, payload),
            HookEvent::Repository(payload) => get_repo_command(&config, payload),
            HookEvent::Release(payload) => get_release_command(&config, payload),
        };
        info!("{:#?}", command.output());
    }
}

fn add_repository_env(command: &mut Command, prefix: &str, repository: &Repository) {
    command.env(format!("{}_ID", prefix), &repository.id.to_string());
    command.env(format!("{}_NAME", prefix), repository.name.as_str());
    command.env(
        format!("{}_FULL_NAME", prefix),
        repository.full_name.as_str(),
    );
    command.env(
        format!("{}_DESCRIPTION", prefix),
        repository.description.as_str(),
    );
    command.env(
        format!("{}_IS_EMPTY", prefix),
        &repository.is_empty.to_string(),
    );
    command.env(
        format!("{}_IS_PRIVATE", prefix),
        &repository.is_private.to_string(),
    );
    command.env(
        format!("{}_IS_FORK", prefix),
        &repository.is_fork.to_string(),
    );
    command.env(
        format!("{}_IS_TEMPLATE", prefix),
        &repository.is_template.to_string(),
    );
    command.env(
        format!("{}_IS_MIRROR", prefix),
        &repository.is_mirror.to_string(),
    );
    command.env(
        format!("{}_SIZE_MIB", prefix),
        &repository.size_mib.to_string(),
    );
    command.env(format!("{}_HTML_URL", prefix), repository.html_url.as_str());
    command.env(format!("{}_SSH_URL", prefix), repository.ssh_url.as_str());
    command.env(
        format!("{}_CLONE_URL", prefix),
        repository.clone_url.as_str(),
    );
    command.env(
        format!("{}_ORIGINAL_URL", prefix),
        repository.original_url.as_str(),
    );
    command.env(format!("{}_WEBSITE", prefix), repository.website.as_str());
    command.env(
        format!("{}_STARS_COUNT", prefix),
        &repository.stars_count.to_string(),
    );
    command.env(
        format!("{}_FORKS_COUNT", prefix),
        &repository.forks_count.to_string(),
    );
    command.env(
        format!("{}_WATCHERS_COUNT", prefix),
        &repository.watchers_count.to_string(),
    );
    command.env(
        format!("{}_OPEN_ISSUES_COUNT", prefix),
        &repository.open_issues_count.to_string(),
    );
    command.env(
        format!("{}_OPEN_PR_COUNTER", prefix),
        &repository.open_pr_counter.to_string(),
    );
    command.env(
        format!("{}_RELEASE_COUNTER", prefix),
        &repository.release_counter.to_string(),
    );
    command.env(
        format!("{}_DEFAULT_BRANCH", prefix),
        repository.default_branch.as_str(),
    );
    command.env(
        format!("{}_IS_ARCHIVED", prefix),
        &repository.is_archived.to_string(),
    );
    command.env(
        format!("{}_CREATED_AT", prefix),
        repository.created_at.as_str(),
    );
    command.env(
        format!("{}_UPDATED_AT", prefix),
        repository.updated_at.as_str(),
    );
    command.env(
        format!("{}_HAS_ISSUES", prefix),
        &repository.has_issues.to_string(),
    );
    command.env(
        format!("{}_HAS_WIKI", prefix),
        &repository.has_wiki.to_string(),
    );
    command.env(
        format!("{}_HAS_PULL_REQUESTS", prefix),
        &repository.has_pull_requests.to_string(),
    );
    command.env(
        format!("{}_HAS_PROJECTS", prefix),
        &repository.has_projects.to_string(),
    );
    command.env(
        format!("{}_IS_WHITESPACE_CONFLICT_IGNORED", prefix),
        &repository.is_whitespace_conflict_ignored.to_string(),
    );
    command.env(
        format!("{}_ALLOW_MERGE_COMMITS", prefix),
        &repository.allow_merge_commits.to_string(),
    );
    command.env(
        format!("{}_ALLOW_REBASE", prefix),
        &repository.allow_rebase.to_string(),
    );
    command.env(
        format!("{}_ALLOW_REBASE_EXPLICIT", prefix),
        &repository.allow_rebase_explicit.to_string(),
    );
    command.env(
        format!("{}_ALLOW_SQUASH_MERGE", prefix),
        &repository.allow_squash_merge.to_string(),
    );
    command.env(
        format!("{}_AVATAR_URL", prefix),
        repository.avatar_url.as_str(),
    );
    command.env(
        format!("{}_IS_INTERNAL", prefix),
        &repository.is_internal.to_string(),
    );

    if let Some(permissions) = &repository.permissions {
        add_permissions_env(command, &format!("{}_PERMISSIONS", prefix), permissions);
    }

    add_user_env(command, &format!("{}_OWNER", prefix), &repository.owner);

    if let Some(internal_tracker) = &repository.internal_tracker {
        add_internal_tracker_env(
            command,
            &format!("{}_INTERNAL_TRACKER", prefix),
            internal_tracker,
        );
    }

    if let Some(external_tracker) = &repository.external_tracker {
        add_external_tracker_env(
            command,
            &format!("{}_EXTERNAL_TRACKER", prefix),
            external_tracker,
        );
    }

    if let Some(external_wiki) = &repository.external_wiki {
        add_external_wiki_env(command, &format!("{}_EXTERNAL_WIKI", prefix), external_wiki);
    }
}

fn add_internal_tracker_env(
    command: &mut Command,
    prefix: &str,
    internal_tracker: &InternalTracker,
) {
    command.env(
        format!("{}_IS_ENABLED", prefix),
        &internal_tracker.is_enabled.to_string(),
    );
    command.env(
        format!("{}_IS_CONTRIBUTOR_ONLY_TIME_TRACKING_ENABLED", prefix),
        internal_tracker
            .is_contributor_only_time_tracking_enabled
            .to_string(),
    );
    command.env(
        format!("{}_IS_ISSUE_DEPENDENCIES_ENABLED", prefix),
        internal_tracker.is_issue_dependencies_enabled.to_string(),
    );
}

fn add_external_wiki_env(command: &mut Command, prefix: &str, external_wiki: &ExternalWiki) {
    command.env(format!("{}_URL", prefix), &external_wiki.url);
}

fn add_external_tracker_env(
    command: &mut Command,
    prefix: &str,
    external_tracker: &ExternalTracker,
) {
    command.env(
        format!("{}_EXTERNAL_TRACKER_URL", prefix),
        external_tracker.external_tracker_url.as_str(),
    );
    command.env(
        format!("{}_EXTERNAL_TRACKER_FORMAT", prefix),
        external_tracker.external_tracker_format.as_str(),
    );
    command.env(
        format!("{}_EXTERNAL_TRACKER_STYLE", prefix),
        external_tracker.external_tracker_style.as_str(),
    );
}

fn add_permissions_env(command: &mut Command, prefix: &str, permissions: &Permissions) {
    command.env(
        format!("{}_HAS_ADMIN", prefix),
        &permissions.has_admin.to_string(),
    );
    command.env(
        format!("{}_HAS_PUSH", prefix),
        &permissions.has_push.to_string(),
    );
    command.env(
        format!("{}_HAS_PULL", prefix),
        &permissions.has_pull.to_string(),
    );
}

fn add_user_env(command: &mut Command, prefix: &str, gitea_user: &GiteaUser) {
    command.env(format!("{}_ID", prefix), &gitea_user.id.to_string());
    command.env(format!("{}_LOGIN", prefix), gitea_user.login.as_str());
    command.env(
        format!("{}_FULL_NAME", prefix),
        gitea_user.full_name.as_str(),
    );
    command.env(format!("{}_EMAIL", prefix), gitea_user.email.as_str());
    command.env(
        format!("{}_AVATAR_URL", prefix),
        gitea_user.avatar_url.as_str(),
    );
    command.env(format!("{}_USERNAME", prefix), gitea_user.username.as_str());
    command.env(format!("{}_LANGUAGE", prefix), gitea_user.language.as_str());
    command.env(
        format!("{}_IS_ADMIN", prefix),
        &gitea_user.is_admin.to_string(),
    );
    command.env(
        format!("{}_LAST_LOGIN", prefix),
        gitea_user.last_login.as_str(),
    );
    command.env(format!("{}_CREATED", prefix), gitea_user.created.as_str());
}

fn add_commit_env(command: &mut Command, prefix: &str, commit: &Commit) {
    command.env(format!("{}_ID", prefix), commit.id.as_str());
    command.env(format!("{}_MESSAGE", prefix), commit.message.as_str());
    command.env(format!("{}_URL", prefix), commit.url.as_str());
    command.env(format!("{}_TIMESTAMP", prefix), commit.timestamp.as_str());

    add_git_user_env(command, &format!("{}_AUTHOR", prefix), &commit.author);
    add_git_user_env(command, &format!("{}_COMMITTER", prefix), &commit.committer);
}

fn add_git_user_env(command: &mut Command, prefix: &str, git_user: &GitUser) {
    command.env(&format!("{}_NAME", prefix), git_user.name.as_str());
    command.env(&format!("{}_EMAIL", prefix), git_user.email.as_str());
    command.env(&format!("{}_USERNAME", prefix), git_user.username.as_str());
}

fn get_push_command(config: &Config, push_event: &PushPayload) -> Command {
    let mut command = Command::new(&config.push);
    // TODO: Read configuration at runtime to determine what to do
    command.env("PUSH_REF_PATH", &push_event.ref_path);
    command.env("PUSH_BEFORE", &push_event.before);
    command.env("PUSH_AFTER", &push_event.after);
    command.env("PUSH_COMPARE_URL", &push_event.compare_url);
    add_repository_env(&mut command, "PUSH_REPOSITORY", &push_event.repository);
    add_user_env(&mut command, "PUSH_PUSHER", &push_event.pusher);
    add_user_env(&mut command, "PUSH_SENDER", &push_event.sender);
    if let Some(commit) = &push_event.head_commit {
        add_commit_env(&mut command, "PUSH_HEAD_COMMIT", commit);
    }
    for (i, commit) in push_event.commits.iter().enumerate() {
        add_commit_env(&mut command, &format!("PUSH_COMMIT_{}", i), commit);
    }
    command
}

fn get_repo_command(config: &Config, repo_payload: &RepositoryPayload) -> Command {
    let mut command = Command::new(&config.repository);
    // TODO: Read configuration at runtime to determine what to do
    command.env("REPOSITORY_ACTION", &repo_payload.action);
    // can be CREATE_X or DELETE_X
    add_repository_env(
        &mut command,
        // TODO: Just use REPOSITORY_X ?
        &format!("{}_REPOSITORY", repo_payload.action.to_uppercase()),
        &repo_payload.repository,
    );
    add_user_env(
        &mut command,
        &format!("{}_ORGANIZATION", repo_payload.action.to_uppercase()),
        &repo_payload.organization,
    );
    add_user_env(
        &mut command,
        &format!("{}_SENDER", repo_payload.action.to_uppercase()),
        &repo_payload.sender,
    );
    command
}

fn get_fork_command(config: &Config, fork: &ForkPayload) -> Command {
    let mut command = Command::new(&config.fork);
    // TODO: Read configuration at runtime to determine what to do
    add_repository_env(&mut command, "FORK_FORKEE", &fork.forkee);
    add_repository_env(&mut command, "FORK_REPOSITORY", &fork.repository);
    add_user_env(&mut command, "FORK_SENDER", &fork.sender);
    command
}

fn get_release_command(config: &Config, release: &ReleasePayload) -> Command {
    let mut command = Command::new(&config.release);
    // TODO: Read configuration at runtime to determine what to do
    command.env("RELEASE_ACTION", &release.action);
    add_release_env(&mut command, "RELEASE_RELEASE", &release.release);
    add_repository_env(&mut command, "RELEASE_REPOSITORY", &release.repository);
    add_user_env(&mut command, "RELEASE_SENDER", &release.sender);
    command
}

fn add_release_env(command: &mut Command, prefix: &str, release: &Release) {
    command.env(&format!("{}_ID", prefix), &release.id.to_string());
    command.env(&format!("{}_TAG_NAME", prefix), &release.tag_name);
    command.env(
        &format!("{}_TARGET_&releaseISH", prefix),
        &release.target_commitish,
    );
    command.env(&format!("{}_NAME", prefix), &release.name);
    command.env(&format!("{}_BODY", prefix), &release.body);
    command.env(&format!("{}_API_URL", prefix), &release.api_url);
    command.env(&format!("{}_HTML_URL", prefix), &release.html_url);
    command.env(&format!("{}_TARBALL_URL", prefix), &release.tarball_url);
    command.env(&format!("{}_ZIPBALL_URL", prefix), &release.zipball_url);
    command.env(
        &format!("{}_IS_DRAFT", prefix),
        &release.is_draft.to_string(),
    );
    command.env(
        &format!("{}_IS_PRERELEASE", prefix),
        &release.is_prerelease.to_string(),
    );
    command.env(&format!("{}_CREATED_AT", prefix), &release.created_at);
    command.env(&format!("{}_PUBLISHED_AT", prefix), &release.published_at);

    add_user_env(command, &format!("{}_AUTHOR", prefix), &release.author);
    // TODO: Put assets in temp folder and provider as paths
    // command.env(&format!("{}_ASSETS", prefix), &release.assets);
}

fn get_create_command(config: &Config, create: &CreatePayload) -> Command {
    let mut command = Command::new(&config.create);
    command.env("CREATE_SHA", &create.sha);
    command.env("CREATE_REF_PATH", &create.ref_path);
    command.env("CREATE_REF_TYPE", &create.ref_type);
    add_repository_env(&mut command, "CREATE_REPOSITORY", &create.repository);
    add_user_env(&mut command, "CREATE_SENDER", &create.sender);
    command
}

fn get_delete_command(config: &Config, delete: &DeletePayload) -> Command {
    let mut command = Command::new(&config.delete);
    // TODO: Read configuration at runtime to determine what to do
    command.env("DELETE_PUSHER_TYPE", &delete.pusher_type);
    command.env("DELETE_REF_PATH", &delete.ref_path);
    command.env("DELETE_REF_TYPE", &delete.ref_type);
    add_repository_env(&mut command, "DELETE_REPOSITORY", &delete.repository);
    add_user_env(&mut command, "DELETE_SENDER", &delete.sender);
    command
}

fn get_issue_command(config: &Config, issue: &IssuePayload) -> Command {
    let mut command = Command::new(&config.issues);
    // TODO: Read configuration at runtime to determine what to do
    command.env("ISSUE_NUMBER", &issue.number.to_string());
    command.env("ISSUE_ACTION", &issue.action);
    if let Some(changes) = &issue.changes {
        add_changes_env(&mut command, "ISSUE_CHANGES", &changes);
    }
    add_issue_env(&mut command, "ISSUE_ISSUE", &issue.issue);
    add_repository_env(&mut command, "ISSUE_REPOSITORY", &issue.repository);
    add_user_env(&mut command, "ISSUE_SENDER", &issue.sender);
    command
}

fn add_issue_env(command: &mut Command, prefix: &str, issue: &Issue) {
    command.env(&format!("{}_ID", prefix), &issue.id.to_string());
    command.env(&format!("{}_API_URL", prefix), &issue.api_url);
    command.env(&format!("{}_HTML_URL", prefix), &issue.html_url);
    command.env(&format!("{}_NUMBER", prefix), &issue.number.to_string());
    command.env(
        &format!("{}_ORIGINAL_AUTHOR", prefix),
        &issue.original_author,
    );
    command.env(
        &format!("{}_ORIGINAL_AUTHOR_ID", prefix),
        &issue.original_author_id.to_string(),
    );
    command.env(&format!("{}_TITLE", prefix), &issue.title);
    command.env(&format!("{}_BODY", prefix), &issue.body);
    // TODO: Better label handling
    command.env(&format!("{}_LABELS", prefix), issue.labels.join(","));

    if let Some(milestone) = &issue.milestone {
        add_milestone_env(command, &format!("{}_MILESTONE", prefix), &milestone);
    }

    if let Some(assignee) = &issue.assignee {
        add_user_env(command, &format!("{}_ASSIGNEE", prefix), assignee);
    }

    for (i, assignee) in issue.assignees.iter().enumerate() {
        add_user_env(command, &format!("{}_ASSIGNEE_{}", prefix, i), assignee);
    }

    command.env(&format!("{}_STATE", prefix), &issue.state);
    command.env(
        &format!("{}_IS_LOCKED", prefix),
        &issue.is_locked.to_string(),
    );

    command.env(
        &format!("{}_COMMENT_COUNT", prefix),
        &issue.comment_count.to_string(),
    );

    command.env(&format!("{}_CREATED_AT", prefix), &issue.created_at);
    command.env(&format!("{}_UPDATED_AT", prefix), &issue.updated_at);
    if let Some(date) = &issue.closed_at {
        command.env(&format!("{}_CLOSED_AT", prefix), date);
    }
    if let Some(date) = &issue.due_date {
        command.env(&format!("{}_DUE_DATE", prefix), date);
    }

    if let Some(pr_meta) = &issue.pull_request_meta {
        add_pull_request_meta_env(command, &format!("{}_PULL_REQUEST", prefix), &pr_meta);
    }
    if let Some(repo_meta) = &issue.repository_meta {
        add_repository_meta_env(command, &format!("{}_REPOSITORY", prefix), &repo_meta);
    }
    add_user_env(command, &format!("{}_USER", prefix), &issue.user);
}

fn add_milestone_env(command: &mut Command, prefix: &str, milestone: &Milestone) {
    command.env(&format!("{}_ID", prefix), &milestone.id.to_string());
    command.env(&format!("{}_TITLE", prefix), &milestone.title);
    command.env(&format!("{}_DESCRIPTION", prefix), &milestone.description);
    command.env(&format!("{}_STATE", prefix), &milestone.state);
    command.env(
        &format!("{}_OPEN_ISSUES", prefix),
        &milestone.open_issues.to_string(),
    );
    command.env(
        &format!("{}_CLOSED_ISSUES", prefix),
        &milestone.closed_issues.to_string(),
    );
    command.env(&format!("{}_CREATED_AT", prefix), &milestone.created_at);
    command.env(&format!("{}_UPDATED_AT", prefix), &milestone.updated_at);
    command.env(&format!("{}_CLOSED_AT", prefix), &milestone.closed_at);
    command.env(&format!("{}_DUE_ON", prefix), &milestone.due_on);
}

fn add_pull_request_meta_env(
    command: &mut Command,
    prefix: &str,
    pull_request_meta: &PullRequestMeta,
) {
    command.env(
        &format!("{}_IS_MERGED", prefix),
        &pull_request_meta.is_merged.to_string(),
    );
    command.env(
        &format!("{}_TIME_MERGED", prefix),
        &pull_request_meta.time_merged.to_string(),
    );
}

fn add_repository_meta_env(command: &mut Command, prefix: &str, repository_meta: &RepositoryMeta) {
    command.env(&format!("{}_ID", prefix), &repository_meta.id.to_string());
    command.env(&format!("{}_NAME", prefix), &repository_meta.name);
    command.env(&format!("{}_OWNER", prefix), &repository_meta.owner);
    command.env(&format!("{}_FULL_NAME", prefix), &repository_meta.full_name);
}

fn add_changes_env(command: &mut Command, prefix: &str, changes: &Changes) {
    if let Some(title) = &changes.title {
        command.env(&format!("{}_TITLE", prefix), &title.from);
    }
    if let Some(body) = &changes.body {
        command.env(&format!("{}_BODY", prefix), &body.from);
    }
    if let Some(ref_path) = &changes.ref_path {
        command.env(&format!("{}_REF_PATH", prefix), &ref_path.from);
    }
}

fn add_comment_env(command: &mut Command, prefix: &str, comment: &Comment) {
    command.env(&format!("{}_ID", prefix), &comment.id.to_string());
    command.env(&format!("{}_HTML_URL", prefix), &comment.html_url);
    command.env(
        &format!("{}_PULL_REQUEST_URL", prefix),
        &comment.pull_request_url,
    );
    command.env(&format!("{}_ISSUE_URL", prefix), &comment.issue_url);
    command.env(
        &format!("{}_ORIGINAL_AUTHOR", prefix),
        &comment.original_author,
    );
    command.env(
        &format!("{}_ORIGINAL_AUTHOR_ID", prefix),
        &comment.original_author_id.to_string(),
    );
    command.env(&format!("{}_COMMENT_TEXT", prefix), &comment.comment_text);
    command.env(&format!("{}_CREATED_AT", prefix), &comment.created_at);
    command.env(&format!("{}_UPDATED_AT", prefix), &comment.updated_at);

    add_user_env(command, &format!("{}_USER", prefix), &comment.user);
}

fn get_issue_comment_command(config: &Config, issue_comment: &IssueCommentPayload) -> Command {
    let mut command = Command::new(&config.issue_comment);
    // TODO: Read configuration at runtime to determine what to do
    command.env("ISSUE_COMMENT_ACTION", &issue_comment.action.to_string());
    add_comment_env(
        &mut command,
        "ISSUE_COMMENT_COMMENT",
        &issue_comment.comment,
    );
    add_issue_env(&mut command, "ISSUE_COMMENT_ISSUE", &issue_comment.issue);

    if let Some(changes) = &issue_comment.changes {
        add_changes_env(&mut command, "ISSUE_COMMENT_CHANGES", &changes);
    }

    command.env("ISSUE_COMMENT_IS_PULL", &issue_comment.is_pull.to_string());
    add_repository_env(
        &mut command,
        "ISSUE_COMMENT_REPOSITORY",
        &issue_comment.repository,
    );
    add_user_env(&mut command, "ISSUE_COMMENT_SENDER", &issue_comment.sender);

    command
}

fn get_pull_request_command(config: &Config, pull_request: &PullRequestPayload) -> Command {
    let mut command = Command::new(&config.pull_request);
    // TODO: Read configuration at runtime to determine what to do
    command.env("PULL_REQUEST_ACTION", &pull_request.action);
    command.env("PULL_REQUEST_NUMBER", &pull_request.number.to_string());
    add_pull_request_env(
        &mut command,
        "PULL_REQUEST_PULL_REQUEST",
        &pull_request.pull_request,
    );

    if let Some(changes) = &pull_request.changes {
        add_changes_env(&mut command, "PULL_REQUEST_CHANGES", &changes);
    }

    add_repository_env(
        &mut command,
        "PULL_REQUEST_REPOSITORY",
        &pull_request.repository,
    );
    add_user_env(&mut command, "PULL_REQUEST_SENDER", &pull_request.sender);
    command
}

fn add_pull_request_env(command: &mut Command, prefix: &str, pull_request: &PullRequest) {
    command.env(&format!("{}_ID", prefix), &pull_request.id.to_string());
    command.env(&format!("{}_URL", prefix), &pull_request.url);
    command.env(
        &format!("{}_NUMBER", prefix),
        &pull_request.number.to_string(),
    );
    add_user_env(command, &format!("{}_USER", prefix), &pull_request.user);
    command.env(&format!("{}_TITLE", prefix), &pull_request.title);
    command.env(&format!("{}_BODY", prefix), &pull_request.body);
    // TODO: labels: Vec<Label>

    if let Some(milestone) = &pull_request.milestone {
        add_milestone_env(command, &format!("{}_MILESTONE", prefix), &milestone);
    }

    if let Some(assignee) = &pull_request.assignee {
        add_user_env(command, &format!("{}_ASSIGNEE", prefix), assignee);
    }

    for (i, assignee) in pull_request.assignees.iter().enumerate() {
        add_user_env(command, &format!("{}_ASSIGNEE_{}", prefix, i), assignee);
    }
    command.env(&format!("{}_STATE", prefix), &pull_request.state);
    command.env(
        &format!("{}_IS_LOCKED", prefix),
        &pull_request.is_locked.to_string(),
    );
    command.env(
        &format!("{}_COMMENTS", prefix),
        &pull_request.comment_count.to_string(),
    );
    command.env(&format!("{}_HTML_URL", prefix), &pull_request.html_url);
    command.env(&format!("{}_DIFF_URL", prefix), &pull_request.diff_url);
    command.env(&format!("{}_PATCH_URL", prefix), &pull_request.patch_url);
    command.env(
        &format!("{}_IS_MERGEABLE", prefix),
        &pull_request.is_mergeable.to_string(),
    );
    command.env(
        &format!("{}_IS_MERGED", prefix),
        &pull_request.is_merged.to_string(),
    );
    if let Some(date) = &pull_request.merged_at {
        command.env(&format!("{}_MERGED_AT", prefix), date);
    }
    if let Some(sha) = &pull_request.merge_commit_sha {
        command.env(&format!("{}_MERGE_COMMIT_SHA", prefix), sha);
    }
    if let Some(user) = &pull_request.merged_by {
        add_user_env(command, &format!("{}_MERGED_BY", prefix), user);
    }
    if let Some(branch) = &pull_request.base {
        add_pr_branch_info(command, &format!("{}_BASE", prefix), branch);
    }
    if let Some(branch) = &pull_request.head {
        add_pr_branch_info(command, &format!("{}_HEAD", prefix), branch);
    }
    command.env(&format!("{}_MERGE_BASE", prefix), &pull_request.merge_base);

    if let Some(date) = &pull_request.due_date {
        command.env(&format!("{}_DUE_DATE", prefix), date);
    }
    if let Some(date) = &pull_request.created_at {
        command.env(&format!("{}_CREATED_AT", prefix), date);
    }
    if let Some(date) = &pull_request.updated_at {
        command.env(&format!("{}_UPDATED_AT", prefix), date);
    }
    if let Some(date) = &pull_request.closed_at {
        command.env(&format!("{}_CLOSED_AT", prefix), date);
    }
}

fn add_pr_branch_info(command: &mut Command, prefix: &str, pr_branch: &PRBranchInfo) {
    command.env(&format!("{}_LABEL", prefix), &pr_branch.label);
    command.env(&format!("{}_REF_PATH", prefix), &pr_branch.ref_path);
    command.env(&format!("{}_SHA", prefix), &pr_branch.sha);
    command.env(
        &format!("{}_REPO_ID", prefix),
        &pr_branch.repo_id.to_string(),
    );
    add_repository_env(
        command,
        &format!("{}_REPOSITORY", prefix),
        &pr_branch.repository,
    );
}
