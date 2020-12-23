use crate::struct_helpers::*;
use crate::Config;
use luffy_gitea::payloads::*;
use std::process::Command;

pub(crate) fn get_push_command(config: &Config, push_event: &PushPayload) -> Command {
    let mut command = Command::new(&config.push);
    // TODO: Read configuration at runtime to determine what to do
    command.env("REF_PATH", &push_event.ref_path);
    command.env("BEFORE", &push_event.before);
    command.env("AFTER", &push_event.after);
    command.env("COMPARE_URL", &push_event.compare_url);
    add_repository_env(&mut command, "REPOSITORY", &push_event.repository);
    add_user_env(&mut command, "PUSHER", &push_event.pusher);
    add_user_env(&mut command, "SENDER", &push_event.sender);
    if let Some(commit) = &push_event.head_commit {
        add_commit_env(&mut command, "HEAD_COMMIT", commit);
    }
    for (i, commit) in push_event.commits.iter().enumerate() {
        add_commit_env(&mut command, &format!("COMMIT_{}", i), commit);
    }
    command
}

pub(crate) fn get_repo_command(config: &Config, repo_payload: &RepositoryPayload) -> Command {
    let mut command = Command::new(&config.repository);
    // TODO: Read configuration at runtime to determine what to do
    command.env("ACTION", &repo_payload.action);
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

pub(crate) fn get_fork_command(config: &Config, fork: &ForkPayload) -> Command {
    let mut command = Command::new(&config.fork);
    // TODO: Read configuration at runtime to determine what to do
    add_repository_env(&mut command, "FORKEE", &fork.forkee);
    add_repository_env(&mut command, "REPOSITORY", &fork.repository);
    add_user_env(&mut command, "SENDER", &fork.sender);
    command
}

pub(crate) fn get_release_command(config: &Config, release: &ReleasePayload) -> Command {
    let mut command = Command::new(&config.release);
    // TODO: Read configuration at runtime to determine what to do
    command.env("ACTION", &release.action);
    add_release_env(&mut command, "RELEASE", &release.release);
    add_repository_env(&mut command, "REPOSITORY", &release.repository);
    add_user_env(&mut command, "SENDER", &release.sender);
    command
}

pub(crate) fn get_create_command(config: &Config, create: &CreatePayload) -> Command {
    let mut command = Command::new(&config.create);
    command.env("SHA", &create.sha);
    command.env("REF_PATH", &create.ref_path);
    command.env("REF_TYPE", &create.ref_type);
    add_repository_env(&mut command, "REPOSITORY", &create.repository);
    add_user_env(&mut command, "SENDER", &create.sender);
    command
}

pub(crate) fn get_delete_command(config: &Config, delete: &DeletePayload) -> Command {
    let mut command = Command::new(&config.delete);
    // TODO: Read configuration at runtime to determine what to do
    command.env("PUSHER_TYPE", &delete.pusher_type);
    command.env("REF_PATH", &delete.ref_path);
    command.env("REF_TYPE", &delete.ref_type);
    add_repository_env(&mut command, "REPOSITORY", &delete.repository);
    add_user_env(&mut command, "SENDER", &delete.sender);
    command
}

pub(crate) fn get_issue_command(config: &Config, issue: &IssuePayload) -> Command {
    let mut command = Command::new(&config.issues);
    // TODO: Read configuration at runtime to determine what to do
    command.env("NUMBER", &issue.number.to_string());
    command.env("ACTION", &issue.action);
    if let Some(changes) = &issue.changes {
        add_changes_env(&mut command, "CHANGES", &changes);
    }
    add_issue_env(&mut command, "ISSUE", &issue.issue);
    add_repository_env(&mut command, "REPOSITORY", &issue.repository);
    add_user_env(&mut command, "SENDER", &issue.sender);
    command
}

pub(crate) fn get_issue_comment_command(
    config: &Config,
    issue_comment: &IssueCommentPayload,
) -> Command {
    let mut command = Command::new(&config.issue_comment);
    // TODO: Read configuration at runtime to determine what to do
    command.env("ACTION", &issue_comment.action.to_string());
    add_comment_env(&mut command, "COMMENT", &issue_comment.comment);
    add_issue_env(&mut command, "ISSUE", &issue_comment.issue);

    if let Some(changes) = &issue_comment.changes {
        add_changes_env(&mut command, "CHANGES", &changes);
    }

    command.env("IS_PULL", &issue_comment.is_pull.to_string());
    add_repository_env(&mut command, "REPOSITORY", &issue_comment.repository);
    add_user_env(&mut command, "SENDER", &issue_comment.sender);

    command
}

pub(crate) fn get_pull_request_command(
    config: &Config,
    pull_request: &PullRequestPayload,
) -> Command {
    let mut command = Command::new(&config.pull_request);
    // TODO: Read configuration at runtime to determine what to do
    command.env("ACTION", &pull_request.action);
    command.env("NUMBER", &pull_request.number.to_string());
    add_pull_request_env(&mut command, "PULL_REQUEST", &pull_request.pull_request);

    if let Some(changes) = &pull_request.changes {
        add_changes_env(&mut command, "CHANGES", &changes);
    }

    add_repository_env(&mut command, "REPOSITORY", &pull_request.repository);
    add_user_env(&mut command, "SENDER", &pull_request.sender);
    command
}
