pub trait Service<T, E> {
    fn event_header_name() -> &'static str;
    fn parse_hook_event(hook_event_type: &str, hook_event_body: &str) -> Result<T, E>;
}

pub trait Handler<T> {
    fn handle_event(event_type: T);
}
