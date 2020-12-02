use clap::ArgMatches;
use nameof::name_of;
use std::convert::TryFrom;
use std::num::ParseIntError;

#[derive(Debug)]
pub(crate) struct ServerConfig {
    pub(crate) address: String,
    pub(crate) port: u16,
}

impl<'a> TryFrom<ArgMatches<'a>> for ServerConfig {
    type Error = ParseIntError;
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
}
