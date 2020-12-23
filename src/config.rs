use clap::ArgMatches;
use log::*;
use nameof::name_of;
use std::net::SocketAddr;

#[derive(Debug)]
pub(crate) struct ServerConfig {
    pub addr: SocketAddr,
    pub allowed_hosts: Vec<String>,
    pub config_path: String,
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

        let allowed_hosts = arg_matches
            .value_of(name_of!(allowed_hosts in ServerConfig))
            .expect("should have defaulted if not provided");
        info!("{:?}", allowed_hosts);
        let allowed_hosts = serde_json::from_str(allowed_hosts).expect("not a JSON array of hosts");

        let config_path = arg_matches
            .value_of(name_of!(config_path in ServerConfig))
            .expect("path of cli command config must be specified")
            .to_owned();

        ServerConfig {
            addr,
            allowed_hosts,
            config_path,
        }
    }
}
