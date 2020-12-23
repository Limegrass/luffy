mod hook_handlers;
mod struct_helpers;

use async_trait::async_trait;
use hook_handlers::*;
use log::*;
use luffy_core::Handler;
use luffy_gitea::HookEvent;
use serde::Deserialize;
use std::fs::read_to_string;

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
        let config: Config = serde_json::from_str(&config_string).expect("but really though");

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
