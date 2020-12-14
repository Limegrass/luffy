mod config;

use clap::{App, Arg};
use config::ServerConfig;
use log::*;
use nameof::name_of;
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
            Arg::with_name(name_of!(allowed_host in ServerConfig))
                .short("h")
                .long("allowed_hosts")
                .value_name("ALLOWED_HOSTS")
                .help("host names allowed to send requests")
                .takes_value(true)
                .default_value(r#"127.0.0.1:9669"#),
        )
        .get_matches();
    info!("{:?}", arg_matches);

    let config = ServerConfig::from(arg_matches);
    info!("{:?}", config);
    let host = warp::host::exact(&format!("localhost:{}", config.addr.port()))
        .or(warp::host::exact(&config.allowed_host));

    let root = host.map(|_| "Hello world!");
    warp::serve(root).run(config.addr).await;
}
