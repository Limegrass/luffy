mod config;

use clap::{App, Arg};
use config::ServerConfig;
use log::*;
use nameof::name_of;
use trello_git_webhook::gitea::{GitEvent, PushEvent, PUSH_EVENT_HEADER_NAME};
use warp::Filter;

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
                .default_value(r#"["localhost:9669"]"#),
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

    let event_filter = warp::header(PUSH_EVENT_HEADER_NAME).map(|event: GitEvent| match event {
        GitEvent::Push => "test push 123".to_owned(),
        _ => format!("other implemented thing: {:?}", event),
    });

    let tro = warp_hosts
        .and(warp::post())
        .and(warp::path("trello"))
        .and(warp::body::json())
        .and(event_filter)
        .map(|push_event: PushEvent, event_name| {
            info!("{}", event_name);
            warp::reply::json(&push_event)
        });
    warp::serve(tro).run(config.addr).await;
}
