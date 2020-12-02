mod config;

use actix_web::{error::ResponseError, middleware, web, App as ServerApp, HttpRequest, HttpServer};
use clap::{App, Arg};
use config::ServerConfig;
use nameof::name_of;
use std::fmt::Display;

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[derive(Debug)]
struct Error {
    error: anyhow::Error,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl ResponseError for Error {}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Error {
        Error { error }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let arg_matches = App::new("Trello Git Webhook")
        .version("0.1.0")
        .author("James N. <james@niis.me>")
        .about("Updates Trello on git push")
        .arg(
            Arg::with_name(name_of!(address in ServerConfig))
                .short("a")
                .long("address")
                .value_name("ADDRESS")
                .help("ip address to bind to")
                .takes_value(true)
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::with_name(name_of!(port in ServerConfig))
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("port on the ip address to bind to")
                .takes_value(true)
                .default_value("9669"), // TODO: limited range
        )
        .get_matches();
    let config = ServerConfig::from(arg_matches);

    HttpServer::new(|| {
        ServerApp::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind(format!("{}:{}", config.address, config.port))?
    .run()
    .await
}
