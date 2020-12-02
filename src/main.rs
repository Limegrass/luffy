mod config;

use anyhow::Result;
use clap::{App, Arg};
use config::ServerConfig;
use nameof::name_of;
use std::net::TcpListener;
use std::net::TcpStream;
use std::{convert::TryFrom, io::prelude::*};

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
        log::info!("Connection established: {:?}", stream);
        handle_connection(stream)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    log::info!("{}", String::from_utf8_lossy(&buffer));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
