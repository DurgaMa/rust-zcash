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
            panic!("testnet not implemented yet");
        }

        if matches.is_present("regtest") {
            config.regtest = true;
            panic!("regtest not implemented yet");
        }

        if matches.is_present("rpcwait") {
            config.rpcwait = true;
            panic!("rpcwait not implemented yet");
        }

        if matches.is_present("rpcssl") {
            config.rpcssl = true;
            panic!("rpcssl not implemented yet");
        }

        // Retrieve the other options
        if let Some(ref file) = matches.value_of("conf") {
            error!("conf not implemented yet");
            config.conf = file;
        }

        if let Some(ref dir) = matches.value_of("datadir") {
            error!("datadir not implemented yet");
            config.datadir = dir;
        }

        if let Some(ref ip) = matches.value_of("rpcconnect") {
            config.rpcconnect = ip;
        }

        if let Some(ref port) = matches.value_of("rpcport") {
            config.rpcport = port;
        }

        if let Some(ref user) = matches.value_of("rpcuser") {
            config.rpcuser = user;
        }

        if let Some(ref pw) = matches.value_of("rpcpassword") {
            config.rpcpassword = pw;
        }

        config
    }
}
