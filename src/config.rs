use clap::ArgMatches;

pub struct Config<'a> {
    testnet: bool,
    regtest: bool,
    rpcwait: bool,
    rpcssl: bool,
    conf: &'a str,
    datadir: &'a str,
    pub rpcconnect: &'a str,
    pub rpcport: &'a str,
    pub rpcuser: &'a str,
    pub rpcpassword: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a ArgMatches) -> Config<'a> {
        let mut config = Config {
            testnet: false,
            regtest: false,
            rpcwait: false,
            rpcssl: false,
            conf: "zcash.conf",
            datadir: "",
            rpcconnect: "127.0.0.1",
            rpcport: "8232",
            rpcuser: "",
            rpcpassword: "",
        };

        // Retrieve the flags
        if matches.is_present("testnet") {
            config.testnet = true;
            unimplemented!();
        }

        if matches.is_present("regtest") {
            config.regtest = true;
            unimplemented!();
        }

        if matches.is_present("rpcwait") {
            config.rpcwait = true;
            unimplemented!();
        }

        if matches.is_present("rpcssl") {
            config.rpcssl = true;
            unimplemented!();
        }

        // Retrieve the other options
        if let Some(file) = matches.value_of("conf") {
            config.conf = file;
            unimplemented!();
        }

        if let Some(dir) = matches.value_of("datadir") {
            config.datadir = dir;
            unimplemented!();
        }

        if let Some(ip) = matches.value_of("rpcconnect") {
            config.rpcconnect = ip;
        }

        if let Some(port) = matches.value_of("rpcport") {
            config.rpcport = port;
        }

        if let Some(user) = matches.value_of("rpcuser") {
            config.rpcuser = user;
        }

        if let Some(pw) = matches.value_of("rpcpassword") {
            config.rpcpassword = pw;
        }

        config
    }
}
