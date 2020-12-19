// https://github.com/go-gitea/gitea/blob/master/modules/notification/webhook/webhook.go

pub mod payloads;
pub mod structs;

use crate::core::Service;
use payloads::{HookEvent, JsonParseError};
use std::convert::TryInto;

pub struct Gitea;
impl Service<HookEvent, String> for Gitea {
    fn event_header_name(&self) -> &'static str {
        "X-Gitea-Event"
    }

    fn parse_hook_event(
        &self,
        hook_event_type: &str,
        hook_event_body: &str,
    ) -> Result<HookEvent, JsonParseError> {
        match hook_event_type {
            "create" => Ok(HookEvent::Create(hook_event_body.try_into()?)),
            "delete" => Ok(HookEvent::Delete(hook_event_body.try_into()?)),
            "fork" => Ok(HookEvent::Fork(hook_event_body.try_into()?)),
            "issues" => Ok(HookEvent::Issues(hook_event_body.try_into()?)),
            "issue_comment" => Ok(HookEvent::IssueComment(hook_event_body.try_into()?)),
            "push" => Ok(HookEvent::Push(hook_event_body.try_into()?)),
            "pull_request" => Ok(HookEvent::PullRequest(hook_event_body.try_into()?)),
            "repository" => Ok(HookEvent::Repository(hook_event_body.try_into()?)),
            "release" => Ok(HookEvent::Release(hook_event_body.try_into()?)),
            _ => Err(String::from("unrecognized git event")),
        }
    }
}
