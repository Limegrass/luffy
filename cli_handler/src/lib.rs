mod hook_handlers;
mod struct_helpers;

use async_trait::async_trait;
use hook_handlers::*;
use log::*;
use luffy_core::Handler;
use luffy_gitea::HookEvent;
use serde::Deserialize;

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
    #[serde(default)]
    create: String,
    #[serde(default)]
    delete: String,
    #[serde(default)]
    fork: String,
    #[serde(default)]
    issues: String,
    #[serde(default)]
    issue_comment: String,
    #[serde(default)]
    push: String,
    #[serde(default)]
    pull_request: String,
    #[serde(default)]
    repository: String,
    #[serde(default)]
    release: String,
}

// TODO: Make the custom derive, if it's possible
// (but it's fucking impossible)
#[async_trait]
impl Handler<HookEvent> for GiteaCliHandler {
    async fn handle_event(&self, event: &HookEvent) {
        // TODO: Maybe get rid of the event type prefixing in the "handle"
        let config_string = std::fs::read_to_string(&self.config_path).expect("TODO: return Err");
        let config: Config = serde_json::from_str(&config_string).expect("but really though");

        let mut command = match event {
            HookEvent::Create(payload) => get_create_command(&config.create, payload),
            HookEvent::Delete(payload) => get_delete_command(&config.delete, payload),
            HookEvent::Fork(payload) => get_fork_command(&config.fork, payload),
            HookEvent::Issues(payload) => get_issue_command(&config.issues, payload),
            HookEvent::IssueComment(payload) => {
                get_issue_comment_command(&config.issue_comment, payload)
            }
            HookEvent::PullRequest(payload) => {
                get_pull_request_command(&config.pull_request, payload)
            }
            HookEvent::Push(payload) => get_push_command(&config.push, payload),
            HookEvent::Repository(payload) => get_repo_command(&config.repository, payload),
            HookEvent::Release(payload) => get_release_command(&config.release, payload),
        };
        info!("{:#?}", command.output());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_empty_string() {
        let config_string = "{}";
        let config: Config = serde_json::from_str(config_string).expect("Serde config error");
        assert_eq!("", &config.create);
        assert_eq!("", &config.delete);
        assert_eq!("", &config.fork);
        assert_eq!("", &config.issues);
        assert_eq!("", &config.issue_comment);
        assert_eq!("", &config.pull_request);
        assert_eq!("", &config.push);
        assert_eq!("", &config.repository);
        assert_eq!("", &config.release);
    }
}
