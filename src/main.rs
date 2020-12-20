mod config;

use async_trait::async_trait;
use clap::{App, Arg};
use config::ServerConfig;
use log::*;
use luffy_core::{Handler, Service};
use luffy_gitea::{
    payloads::{HookEvent, PushPayload},
    GiteaService,
};
use nameof::name_of;
use tokio::process::Command;
use warp::{hyper::body::Bytes, Buf, Filter};

#[tokio::main]
async fn main() {
    env_logger::init();
    let arg_matches = App::new("Trello Git Webhook")
        .version("0.1.0")
        .author("James N. <james@niis.me>")
        .about("Updates Trello on git push")
        .arg(
            Arg::with_name(name_of!(addr in ServerConfig))
                .short("a")
                .long("address")
                .value_name("ADDRESS")
                .help("ip address to bind to")
                .takes_value(true)
                .default_value("127.0.0.1:9669"),
        )
        .arg(
            // not sure if the default is a vector
            // if someone pointed localhost to the server.
            Arg::with_name(name_of!(allowed_hosts in ServerConfig))
                .short("h")
                .long("allowed_hosts")
                .value_name("ALLOWED_HOSTS")
                .help("host names allowed to send requests")
                .takes_value(true)
                .default_value(r#"[]"#),
        )
        .get_matches();
    info!("{:?}", arg_matches);

    let config = ServerConfig::from(arg_matches);
    info!("{:?}", config);

    let warp_hosts = config.allowed_hosts.iter().fold(
        // always allow the loopback to connect.
        warp::host::exact(&format!("127.0.0.1:{}", config.addr.port())).boxed(),
        |allowed_hosts, hostname| {
            allowed_hosts
                .or(warp::host::exact(hostname))
                .unify()
                .boxed()
        },
    );

    let gitea = GiteaService;
    let event_header_name = gitea.event_header_name();
    let event_filter = warp::header(event_header_name);

    // TODO: Decide on how to map commits to trello board updates
    //      Commits -> In progress
    //          Must be mapped to the right card somehow (schema)
    //      PR Comments -> In progress and mirror comments in trello
    //          Have to check what I can use to map this
    //      PR Merge -> Complete
    //          Have to check what I can use to map this, or use a merge commit with message schema

    let tro = warp_hosts
        .and(warp::post())
        .and(warp::path("trello"))
        .and(event_filter)
        .and(warp::body::bytes())
        .and_then(handle_event_async);
    warp::serve(tro).run(config.addr).await;
}

async fn handle_event_async(
    hook_event_name: String,
    hook_event_body: Bytes,
) -> Result<warp::reply::Json, warp::Rejection> {
    info!("{}", hook_event_name);
    let mut cards = std::process::Command::new("tro");
    cards.arg("show");
    info!("{:?}", cards.output());
    let body = String::from_utf8(hook_event_body.bytes().iter().map(|b| b.clone()).collect())
        .expect("fuck it i'll fix it later");
    let gitea = GiteaService;
    let event = gitea
        .parse_hook_event(&hook_event_name, &body)
        .expect("fuck it i'll fix it later");
    let process_handler = GiteaCliHandler;
    process_handler.handle_event(&event).await;
    Ok(warp::reply::json(&event))
}

pub struct GiteaCliHandler;

#[async_trait]
impl Handler<HookEvent> for GiteaCliHandler {
    async fn handle_event(&self, event: &HookEvent) {
        match event {
            HookEvent::Push(push_event) => handle_push_event(push_event).await,
            _ => (),
        }
    }
}

async fn handle_push_event(push_event: &PushPayload) {
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
        "PUSH_REPOSITORY_IS_PRIVATE",
        push_event.repository.is_private.to_string(),
    );
    command.env(
        "PUSH_REPOSITORY_IS_FORK",
        push_event.repository.is_fork.to_string(),
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
