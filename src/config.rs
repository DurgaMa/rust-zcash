use clap::ArgMatches;

pub struct Config<'a> {
    testnet: bool,
    regtest: bool,
    rpcwait: bool,
    rpcssl: bool,
    conf: &'a str,
    datadir: &'a str,
    rpcconnect: &'a str,
    rpcport: &'a str,
    rpcuser: &'a str,
    rpcpassword: &'a str,
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
            error!("testnet not implemented yet");
            config.testnet = true;
        }

        if matches.is_present("regtest") {
            error!("regtest not implemented yet");
            config.regtest = true;
        }

        if matches.is_present("rpcwait") {
            error!("rpcwait not implemented yet");
            config.rpcwait = true;
        }

        if matches.is_present("rpcssl") {
            error!("rpcssl not implemented yet");
            config.rpcssl = true;
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
            error!("rpcconnect not implemented yet");
            config.rpcconnect = ip;
        }

        if let Some(ref port) = matches.value_of("rpcport") {
            error!("rpcport not implemented yet");
            config.rpcport = port;
        }

        if let Some(ref user) = matches.value_of("rpcuser") {
            error!("rpcuser not implemented yet");
            config.rpcuser = user;
        }

        if let Some(ref pw) = matches.value_of("rpcpassword") {
            error!("rpcpassword not implemented yet");
            config.rpcpassword = pw;
        }

        config
    }
}
