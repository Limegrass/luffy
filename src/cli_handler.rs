use crate::core::Handler;
use crate::gitea::{HookEvent, PushEvent};
use async_trait::async_trait;
use log::*;
use tokio::process::Command;

pub struct CliHandler;

#[async_trait]
impl Handler<HookEvent> for CliHandler {
    async fn handle_event(&self, event: &HookEvent) {
        match event {
            HookEvent::Push(push_event) => handle_push_event(push_event).await,
            _ => (),
        }
    }
}

async fn handle_push_event(push_event: &PushEvent) {
    // loop through the commits and check message
    let mut command = Command::new("echoer");
    // TODO: Read configuration at runtime to determine what to do
    // TODO: split the schema message and include as args
    command.env("PUSH_REF_PATH", &push_event.ref_path);
    command.env("PUSH_BEFORE", &push_event.before);
    command.env("PUSH_AFTER", &push_event.after);
    command.env("PUSH_PUSHER_ID", push_event.pusher.id.to_string());
    command.env("PUSH_PUSHER_LOGIN", &push_event.pusher.login);
    command.env("PUSH_PUSHER_FULL_NAME", &push_event.pusher.full_name);
    command.env("PUSH_PUSHER_EMAIL", &push_event.pusher.email);
    command.env("PUSH_PUSHER_AVATAR_URL", &push_event.pusher.avatar_url);
    command.env("PUSH_PUSHER_USERNAME", &push_event.pusher.username);
    command.env("PUSH_SENDER_ID", push_event.sender.id.to_string());
    command.env("PUSH_SENDER_LOGIN", &push_event.sender.login);
    command.env("PUSH_SENDER_FULL_NAME", &push_event.sender.full_name);
    command.env("PUSH_SENDER_EMAIL", &push_event.sender.email);
    command.env("PUSH_SENDER_AVATAR_URL", &push_event.sender.avatar_url);
    command.env("PUSH_SENDER_USERNAME", &push_event.sender.username);
    command.env("PUSH_REPOSITORY_ID", push_event.repository.id.to_string());
    command.env(
        "PUSH_REPOSITORY_OWNER_ID",
        push_event.repository.owner.id.to_string(),
    );
    command.env(
        "PUSH_REPOSITORY_OWNER_LOGIN",
        &push_event.repository.owner.login,
    );
    command.env(
        "PUSH_REPOSITORY_OWNER_FULL_NAME",
        &push_event.repository.owner.full_name,
    );
    command.env(
        "PUSH_REPOSITORY_OWNER_EMAIL",
        &push_event.repository.owner.email,
    );
    command.env(
        "PUSH_REPOSITORY_OWNER_AVATAR_URL",
        &push_event.repository.owner.avatar_url,
    );
    command.env(
        "PUSH_REPOSITORY_OWNER_USERNAME",
        &push_event.repository.owner.username,
    );
    command.env("PUSH_REPOSITORY_NAME", &push_event.repository.name);
    command.env(
        "PUSH_REPOSITORY_FULL_NAME",
        &push_event.repository.full_name,
    );
    command.env(
        "PUSH_REPOSITORY_DESCRIPTION",
        &push_event.repository.description,
    );
    command.env(
        "PUSH_REPOSITORY_PRIVATE",
        push_event.repository.private.to_string(),
    );
    command.env(
        "PUSH_REPOSITORY_FORK",
        push_event.repository.fork.to_string(),
    );
    command.env("PUSH_REPOSITORY_HTML_URL", &push_event.repository.html_url);
    command.env("PUSH_REPOSITORY_SSH_URL", &push_event.repository.ssh_url);
    command.env(
        "PUSH_REPOSITORY_CLONE_URL",
        &push_event.repository.clone_url,
    );
    command.env("PUSH_REPOSITORY_WEBSITE", &push_event.repository.website);
    command.env(
        "PUSH_REPOSITORY_STARS_COUNT",
        push_event.repository.stars_count.to_string(),
    );
    command.env(
        "PUSH_REPOSITORY_FORKS_COUNT",
        push_event.repository.forks_count.to_string(),
    );
    command.env(
        "PUSH_REPOSITORY_WATCHERS_COUNT",
        push_event.repository.watchers_count.to_string(),
    );
    command.env(
        "PUSH_REPOSITORY_OPEN_ISSUES_COUNT",
        push_event.repository.open_issues_count.to_string(),
    );
    command.env(
        "PUSH_REPOSITORY_DEFAULT_BRANCH",
        &push_event.repository.default_branch,
    );
    command.env(
        "PUSH_REPOSITORY_CREATED_AT",
        &push_event.repository.created_at,
    );
    command.env(
        "PUSH_REPOSITORY_UPDATED_AT",
        &push_event.repository.updated_at,
    );
    command.arg("hi $PUSH_PUSHER_FULL_NAME");
    info!("{:#?}", command.output().await);
}
