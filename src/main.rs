#[macro_use]
extern crate nameof;
use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use log;
use std::convert::TryFrom;
use std::net::TcpListener;
use std::num::ParseIntError;

#[derive(Debug)]
struct ServerConfig {
    address: String,
    port: u16,
}

impl<'a> TryFrom<ArgMatches<'a>> for ServerConfig {
    fn try_from(arg_matches: ArgMatches<'a>) -> Result<Self, Self::Error> {
        Ok(ServerConfig {
            address: arg_matches
                .value_of(name_of!(address in ServerConfig))
                .expect("should have defaulted if not provided")
                .to_owned(),
            port: arg_matches
                .value_of(name_of!(port in ServerConfig))
                .expect("should have defaulted if not provided")
                .parse()?,
        })
    }

    type Error = ParseIntError;
}

pub fn main() -> Result<()> {
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
    let config = ServerConfig::try_from(arg_matches)?;

    let bind_address = format!("{}:{}", config.address, config.port);
    let listener = TcpListener::bind(bind_address)?;
    for stream in listener.incoming() {
        let stream = stream?;
        log::info!("Connection established: {:?}", stream)
    }
    Ok(())
}
