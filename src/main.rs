mod config;

use clap::{App, Arg};
use cli_handler::{GiteaCliHandler, HookHandlerResult};
use config::ServerConfig;
use log::*;
use luffy_core::{Handler, Service};
use luffy_gitea::GiteaService;
use nameof::name_of;
use std::{string::FromUtf8Error, sync::Arc};
use warp::{hyper::body::Bytes, Buf, Filter};

#[tokio::main]
async fn main() {
    env_logger::init();
    let arg_matches = App::new("Trello Git Webhook")
        .version("0.1.0")
        .author("James N. <james@niis.me>")
        .about("General purpose webhook")
        .arg(
            Arg::with_name(name_of!(addr in ServerConfig))
                .short("a")
                .long("address")
                .value_name("IP_ADDRESS:PORT")
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
                .value_name("[ALLOWED_HOST1, ...]")
                .help("host names allowed to send requests")
                .takes_value(true)
                .default_value(r#"[]"#),
        )
        .arg(
            Arg::with_name(name_of!(config_path in ServerConfig))
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Config path")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    info!("{:?}", arg_matches);

    let config = ServerConfig::from(arg_matches);
    info!("{:?}", config);

    let warp_hosts = config.allowed_hosts.iter().fold(
        // always allow the binding address
        warp::host::exact(&format!("{}:{}", config.addr.ip(), config.addr.port())).boxed(),
        |allowed_hosts, hostname| {
            allowed_hosts
                .or(warp::host::exact(hostname))
                .unify()
                .boxed()
        },
    );

    let gitea = Arc::from(GiteaService);
    let event_header_name = gitea.event_header_name();
    let event_filter = warp::header(event_header_name);

    let config_path = config.config_path;

    let hook = warp_hosts
        .and(warp::post())
        .and(event_filter)
        .and(warp::body::bytes())
        .and_then(move |hook_event_name: String, hook_event_body: Bytes| {
            let process_handler = GiteaCliHandler::new(&config_path);
            let gitea = Arc::clone(&gitea);
            async move {
                let body = match String::from_utf8(
                    hook_event_body.bytes().iter().map(|b| b.clone()).collect(),
                ) {
                    Ok(body) => body,
                    Err(error) => {
                        return Err::<warp::reply::Json, warp::Rejection>(Error::from(error).into())
                    }
                };
                let event = match gitea.parse_hook_event(&hook_event_name, &body) {
                    Ok(event) => event,
                    Err(error) => {
                        return Err::<warp::reply::Json, warp::Rejection>(Error::from(error).into())
                    }
                };
                match process_handler.handle_event(&event).await {
                    Ok(result) => match result {
                        HookHandlerResult::ExecutionResult(io_result) => {
                            // TODO: Map this in the Handler error instead
                            let command_result = io_result.map_err(|err| Error::from(err))?;
                            let stdout = match String::from_utf8(command_result.stdout.clone()) {
                                Ok(stdout) => stdout,
                                Err(conversion_error) => format!(
                                    "conversion error [{:?}], could not convert array {:?}",
                                    conversion_error, &command_result.stdout
                                ),
                            };
                            let stderr = match String::from_utf8(command_result.stderr.clone()) {
                                Ok(stderr) => stderr,
                                Err(conversion_error) => format!(
                                    "conversion error [{:?}], could not convert array {:?}",
                                    conversion_error, &command_result.stderr
                                ),
                            };

                            Ok(warp::reply::json(&format!(
                                "status: {:?}, stdout: {}, stderr: {}",
                                command_result.status, stdout, stderr
                            )))
                        }
                        HookHandlerResult::NoOp => Ok(warp::reply::json(&"No op")),
                    },
                    Err(error) => Err(Error::from(error).into()),
                }
            }
        });
    warp::serve(hook).run(config.addr).await;
}

#[derive(Debug)]
enum Error {
    Deserialization(luffy_gitea::Error),
    PayloadBody(FromUtf8Error),
    ExecutionError(String),
    ConfigError(cli_handler::Error),
}

impl From<cli_handler::Error> for Error {
    fn from(error: cli_handler::Error) -> Error {
        Error::ConfigError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::ExecutionError(error.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::PayloadBody(error)
    }
}

impl From<luffy_gitea::Error> for Error {
    fn from(error: luffy_gitea::Error) -> Error {
        Error::Deserialization(error)
    }
}

impl From<Error> for warp::Rejection {
    fn from(rejection: Error) -> Self {
        warp::reject::custom(rejection)
    }
}

impl warp::reject::Reject for Error {}
