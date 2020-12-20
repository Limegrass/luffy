use async_trait::async_trait;

/// A Git Repository Service Provider
pub trait Service<T, E> {
    fn event_header_name(&self) -> &'static str;
    fn parse_hook_event(&self, hook_event_type: &str, hook_event_body: &str) -> Result<T, E>;
}

/// A Handler for Git WebHook Events
#[async_trait]
pub trait Handler<T> {
    async fn handle_event(&self, event: &T);
}
