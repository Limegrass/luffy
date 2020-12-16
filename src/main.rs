mod config;
mod service;

use clap::{App, Arg};
use config::ServerConfig;
use log::*;
use nameof::name_of;
use trello_git_webhook::gitea::{Gitea, HookEvent};
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

    let event_header_name =
        <Gitea as trello_git_webhook::service::Service<HookEvent, String>>::event_header_name();
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
        .map(|hook_event_name: String, hook_event_body: Bytes| {
            info!("{}", hook_event_name);
            let mut cards = std::process::Command::new("tro");
            cards.arg("show");
            info!("{:?}", cards.output());
            let body =
                String::from_utf8(hook_event_body.bytes().iter().map(|b| b.clone()).collect())
                    .expect("fuck it i'll fix it later");
            let event = <Gitea as trello_git_webhook::service::Service<
                        HookEvent,
                        String,
                    >>::parse_hook_event(&hook_event_name, &body).expect("fuck it i'll fix it later");
            warp::reply::json(&event)
        });
    warp::serve(tro).run(config.addr).await;
}
