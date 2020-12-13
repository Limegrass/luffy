use clap::ArgMatches;
use nameof::name_of;
use std::net::SocketAddr;

#[derive(Debug)]
pub(crate) struct ServerConfig {
    pub(crate) addr: SocketAddr,
}

impl<'a> From<ArgMatches<'a>> for ServerConfig {
    fn from(arg_matches: ArgMatches<'a>) -> Self {
        ServerConfig {
            addr: arg_matches
                .value_of(name_of!(addr in ServerConfig))
                .expect("should have defaulted if not provided")
                .to_owned()
                .parse()
                .expect("must be a valid socket addr. eg: 127.0.0.1:8080"),
        }
    }
}
