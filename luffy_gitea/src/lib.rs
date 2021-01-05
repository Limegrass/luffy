// https://github.com/go-gitea/gitea/blob/master/modules/notification/webhook/webhook.go
pub mod payloads;
pub mod structs;

use luffy_core::Service;
use payloads::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub enum HookEvent {
    Create(CreatePayload),
    Delete(DeletePayload),
    Fork(ForkPayload),
    Issues(IssuePayload),
    IssueComment(IssueCommentPayload),
    Push(PushPayload),
    PullRequest(PullRequestPayload),
    Repository(RepositoryPayload),
    Release(ReleasePayload),
}

pub struct GiteaService;
#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "error while deserializing {0}, you may need to report a bug to github.com/limegrass/luffy"
    )]
    PayloadBody(serde_json::Error),
    #[error(
        "unrecognized event type {0}, you may need to report a bug to github.com/limegrass/luffy"
    )]
    EventType(String),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::PayloadBody(error)
    }
}

impl Service<HookEvent, Error> for GiteaService {
    fn event_header_name(&self) -> &'static str {
        "X-Gitea-Event"
    }

    fn parse_hook_event(
        &self,
        hook_event_type: &str,
        hook_event_body: &str,
    ) -> Result<HookEvent, Error> {
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
            _ => Err(Error::EventType(String::from(hook_event_type))),
        }
    }
}
