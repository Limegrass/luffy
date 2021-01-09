mod hook_handlers;
mod struct_helpers;

use async_trait::async_trait;
use hook_handlers::*;
use log::*;
use luffy_core::Handler;
use luffy_gitea::HookEvent;
use serde::de::IntoDeserializer;
use serde::Deserialize;

// https://github.com/serde-rs/serde/issues/1425, modified for blank
fn blank_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

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
    #[serde(default, deserialize_with = "blank_string_as_none")]
    create: Option<String>,
    #[serde(default, deserialize_with = "blank_string_as_none")]
    delete: Option<String>,
    #[serde(default, deserialize_with = "blank_string_as_none")]
    fork: Option<String>,
    #[serde(default, deserialize_with = "blank_string_as_none")]
    issues: Option<String>,
    #[serde(default, deserialize_with = "blank_string_as_none")]
    issue_comment: Option<String>,
    #[serde(default, deserialize_with = "blank_string_as_none")]
    push: Option<String>,
    #[serde(default, deserialize_with = "blank_string_as_none")]
    pull_request: Option<String>,
    #[serde(default, deserialize_with = "blank_string_as_none")]
    repository: Option<String>,
    #[serde(default, deserialize_with = "blank_string_as_none")]
    release: Option<String>,
}

// TODO: Make the custom derive, if it's possible
// (but it's fucking impossible)
#[async_trait]
impl Handler<HookEvent> for GiteaCliHandler {
    async fn handle_event(&self, event: &HookEvent) {
        let config_string = std::fs::read_to_string(&self.config_path).expect("TODO: return Err");
        let config: Config = serde_json::from_str(&config_string).expect("but really though");

        let command = get_command_string(&config, event);
        if let Some(mut command) = command {
            info!("{:#?}", command.output());
        }
    }
}

// No op if empty/defaulted
fn get_command_string(config: &Config, event: &HookEvent) -> Option<std::process::Command> {
    match event {
        HookEvent::Create(payload) => Some(get_create_command(&config.create.as_ref()?, payload)),
        HookEvent::Delete(payload) => Some(get_delete_command(&config.delete.as_ref()?, payload)),
        HookEvent::Fork(payload) => Some(get_fork_command(&config.fork.as_ref()?, payload)),
        HookEvent::Issues(payload) => Some(get_issue_command(&config.issues.as_ref()?, payload)),
        HookEvent::IssueComment(payload) => Some(get_issue_comment_command(
            &config.issue_comment.as_ref()?,
            payload,
        )),
        HookEvent::PullRequest(payload) => Some(get_pull_request_command(
            &config.pull_request.as_ref()?,
            payload,
        )),
        HookEvent::Push(payload) => Some(get_push_command(&config.push.as_ref()?, payload)),
        HookEvent::Repository(payload) => {
            Some(get_repo_command(&config.repository.as_ref()?, payload))
        }
        HookEvent::Release(payload) => {
            Some(get_release_command(&config.release.as_ref()?, payload))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_none(config: &Config) {
        assert_eq!(None, config.create);
        assert_eq!(None, config.delete);
        assert_eq!(None, config.fork);
        assert_eq!(None, config.issues);
        assert_eq!(None, config.issue_comment);
        assert_eq!(None, config.pull_request);
        assert_eq!(None, config.push);
        assert_eq!(None, config.repository);
        assert_eq!(None, config.release);
    }

    #[test]
    fn defaults_none() {
        let config_string = "{}";
        let config: Config = serde_json::from_str(config_string).expect("Serde config error");
        assert_all_none(&config)
    }

    #[test]
    fn empty_string_is_none() {
        let config_string = r#"{
            "create"        : "",
            "delete"        : "",
            "fork"          : "",
            "issues"        : "",
            "issue_comment" : "",
            "pull_request"  : "",
            "push"          : "",
            "repository"    : "",
            "release"       : ""
        }"#;
        let config: Config = serde_json::from_str(config_string).expect("Serde config error");
        assert_all_none(&config)
    }

    #[test]
    fn blank_string_is_none() {
        let config_string = r#"{
            "create"        : " \t\r\n",
            "delete"        : " \t\r\n",
            "fork"          : " \t\r\n",
            "issues"        : " \t\r\n",
            "issue_comment" : " \t\r\n",
            "pull_request"  : " \t\r\n",
            "push"          : " \t\r\n",
            "repository"    : " \t\r\n",
            "release"       : " \t\r\n"
        }"#;
        let config: Config = serde_json::from_str(config_string).expect("Serde config error");
        assert_all_none(&config)
    }
}
