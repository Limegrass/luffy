use clap::ArgMatches;
use nameof::name_of;

#[derive(Debug)]
pub(crate) struct ServerConfig {
    pub(crate) address: String,
    pub(crate) port: u16,
}

impl<'a> From<ArgMatches<'a>> for ServerConfig {
    fn from(arg_matches: ArgMatches<'a>) -> Self {
        ServerConfig {
            address: arg_matches
                .value_of(name_of!(address in ServerConfig))
                .expect("should have defaulted if not provided")
                .to_owned(),
            port: arg_matches
                .value_of(name_of!(port in ServerConfig))
                .expect("should have defaulted if not provided")
                .parse()
                .expect("port should be a number"),
        }
    }
}
