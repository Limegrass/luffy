use clap::ArgMatches;
use log::*;
use nameof::name_of;
use std::net::SocketAddr;

#[derive(Debug)]
pub(crate) struct ServerConfig {
    pub addr: SocketAddr,
    pub allowed_host: String,
}

impl<'a> From<ArgMatches<'a>> for ServerConfig {
    fn from(arg_matches: ArgMatches<'a>) -> Self {
        let addr = arg_matches
            .value_of(name_of!(addr in ServerConfig))
            .expect("should have defaulted if not provided");
        info!("{:?}", addr);
        let addr: SocketAddr = addr
            .to_owned()
            .parse()
            .expect("must be a valid socket addr. eg: 127.0.0.1:8080");

        let allowed_host = arg_matches
            .value_of(name_of!(allowed_host in ServerConfig))
            .expect("should have defaulted if not provided");
        info!("{:?}", allowed_host);

        ServerConfig {
            addr,
            allowed_host: allowed_host.to_owned(),
        }
    }
}
