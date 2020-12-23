use crate::struct_helpers::*;
use crate::Config;
use luffy_gitea::payloads::*;
use std::process::Command;

pub(crate) fn get_push_command(config: &Config, push_event: &PushPayload) -> Command {
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

pub(crate) fn get_repo_command(config: &Config, repo_payload: &RepositoryPayload) -> Command {
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

pub(crate) fn get_fork_command(config: &Config, fork: &ForkPayload) -> Command {
    let mut command = Command::new(&config.fork);
    // TODO: Read configuration at runtime to determine what to do
    add_repository_env(&mut command, "FORK_FORKEE", &fork.forkee);
    add_repository_env(&mut command, "FORK_REPOSITORY", &fork.repository);
    add_user_env(&mut command, "FORK_SENDER", &fork.sender);
    command
}

pub(crate) fn get_release_command(config: &Config, release: &ReleasePayload) -> Command {
    let mut command = Command::new(&config.release);
    // TODO: Read configuration at runtime to determine what to do
    command.env("RELEASE_ACTION", &release.action);
    add_release_env(&mut command, "RELEASE_RELEASE", &release.release);
    add_repository_env(&mut command, "RELEASE_REPOSITORY", &release.repository);
    add_user_env(&mut command, "RELEASE_SENDER", &release.sender);
    command
}

pub(crate) fn get_create_command(config: &Config, create: &CreatePayload) -> Command {
    let mut command = Command::new(&config.create);
    command.env("CREATE_SHA", &create.sha);
    command.env("CREATE_REF_PATH", &create.ref_path);
    command.env("CREATE_REF_TYPE", &create.ref_type);
    add_repository_env(&mut command, "CREATE_REPOSITORY", &create.repository);
    add_user_env(&mut command, "CREATE_SENDER", &create.sender);
    command
}

pub(crate) fn get_delete_command(config: &Config, delete: &DeletePayload) -> Command {
    let mut command = Command::new(&config.delete);
    // TODO: Read configuration at runtime to determine what to do
    command.env("DELETE_PUSHER_TYPE", &delete.pusher_type);
    command.env("DELETE_REF_PATH", &delete.ref_path);
    command.env("DELETE_REF_TYPE", &delete.ref_type);
    add_repository_env(&mut command, "DELETE_REPOSITORY", &delete.repository);
    add_user_env(&mut command, "DELETE_SENDER", &delete.sender);
    command
}

pub(crate) fn get_issue_command(config: &Config, issue: &IssuePayload) -> Command {
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

pub(crate) fn get_issue_comment_command(
    config: &Config,
    issue_comment: &IssueCommentPayload,
) -> Command {
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

pub(crate) fn get_pull_request_command(
    config: &Config,
    pull_request: &PullRequestPayload,
) -> Command {
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
