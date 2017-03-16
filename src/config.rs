use clap::ArgMatches;

#[derive(Builder, Debug)]
pub struct Config {
    testnet: bool,
    regtest: bool,
    rpcwait: bool,
    rpcssl: bool,
    conf: String,
    datadir: String,
    rpcconnect: String,
    rpcport: String,
    pub rpcuser: String,
    pub rpcpassword: String,
}

impl Config {
    pub fn load_config_from_args(matches: &ArgMatches) -> Result<Config, String> {
        ConfigBuilder::default()
            .testnet(matches.is_present("testnet"))
            .regtest(matches.is_present("regtest"))
            .rpcwait(matches.is_present("rpcwait"))
            .rpcssl(matches.is_present("rpc_ssl"))
            .conf(matches.value_of("conf").unwrap_or(""))
            .datadir(matches.value_of("datadir").unwrap_or(""))
            .rpcconnect(matches.value_of("rpcconnect").unwrap_or(""))
            .rpcport(matches.value_of("rpcport").unwrap_or(""))
            .rpcuser(matches.value_of("rpcuser").unwrap_or(""))
            .rpcpassword(matches.value_of("rpcpassword").unwrap_or(""))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{App, AppSettings};

    #[test]
    fn create_default_config() {
        let yaml = load_yaml!("cli.yml");
        let matches =
            App::from_yaml(yaml).unset_setting(AppSettings::SubcommandRequired).get_matches();
        let config = Config::load_config_from_args(&matches).unwrap();
        assert_eq!(config.conf, "zcash.conf");
        assert_eq!(config.rpcconnect, "127.0.0.1");
        assert_eq!(config.rpcport, "8232");
    }
}
