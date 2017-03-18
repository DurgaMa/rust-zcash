use clap::ArgMatches;
use ini::ini;
use std::env;

pub struct Config {
    rpcconnect: String,
    rpcport: String,
    rpcuser: String,
    rpcpassword: String,
}

impl Config {
    pub fn load_from_args(matches: &ArgMatches) -> Result<Config, String> {
        ConfigBuilder::new().load_from_args(matches).build()
    }

    pub fn rpcconnect(&self) -> &String {
        &self.rpcconnect
    }

    pub fn rpcport(&self) -> &String {
        &self.rpcport
    }

    pub fn rpcuser(&self) -> &String {
        &self.rpcuser
    }

    pub fn rpcpassword(&self) -> &String {
        &self.rpcpassword
    }
}

struct ConfigBuilder {
    testnet: bool,
    regtest: bool,
    rpcwait: bool,
    rpcssl: bool,
    rpcconnect: String,
    rpcport: String,
    rpcuser: String,
    rpcpassword: String,

    file_error: String,
    default_rpcport: bool,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            testnet: false,
            regtest: false,
            rpcwait: false,
            rpcssl: false,
            rpcconnect: String::from("127.0.0.1"),
            rpcport: String::from("8232"),
            rpcuser: String::new(),
            rpcpassword: String::new(),
            file_error: String::new(),
            default_rpcport: true,
        }
    }

    fn testnet(&mut self, testnet: bool) -> &mut Self {
        self.testnet = testnet;
        self
    }

    fn regtest(&mut self, regtest: bool) -> &mut Self {
        self.regtest = regtest;
        self
    }

    fn rpcwait(&mut self, rpcwait: bool) -> &mut Self {
        self.rpcwait = rpcwait;
        self
    }

    fn rpcssl(&mut self, rpcssl: bool) -> &mut Self {
        self.rpcssl = rpcssl;
        self
    }

    fn rpcconnect(&mut self, rpcconnect: String) -> &mut Self {
        self.rpcconnect = rpcconnect;
        self
    }

    fn rpcport(&mut self, rpcport: String) -> &mut Self {
        self.rpcport = rpcport;
        self.default_rpcport = false;
        self
    }

    fn rpcuser(&mut self, rpcuser: String) -> &mut Self {
        self.rpcuser = rpcuser;
        self
    }

    fn rpcpassword(&mut self, rpcpassword: String) -> &mut Self {
        self.rpcpassword = rpcpassword;
        self
    }

    fn build(&mut self) -> Result<Config, String> {
        // TODO: Errors are rpcwait or rpcssl flagged, testnet and regtest together
        if (self.testnet || self.regtest) && self.default_rpcport {
            self.rpcport = String::from("18232");
        }

        if self.testnet && self.regtest {
            return Err(String::from("Invalid combination of testnet and regtest"));
        }

        if self.rpcssl {
            return Err(String::from("rpcssl not implemented yet"));
        }

        if self.rpcwait {
            return Err(String::from("rpcwait not implemented yet"));
        }

        if !self.file_error.is_empty() {
            return Err(self.file_error.clone());
        }

        Ok(Config {
               rpcconnect: self.rpcconnect.clone(),
               rpcport: self.rpcport.clone(),
               rpcuser: self.rpcuser.clone(),
               rpcpassword: self.rpcpassword.clone(),
           })
    }

    fn load_from_args(&mut self, matches: &ArgMatches) -> &mut Self {
        let default_dir = format!("{}/.zcash", env::home_dir().unwrap().display());
        let filename = format!("{}/{}",
                               matches.value_of("datadir").unwrap_or(&default_dir),
                               matches.value_of("conf").unwrap_or("zcash.conf"));
        self.load_from_file(&filename);

        if matches.is_present("testnet") {
            self.testnet(true);
        }
        if matches.is_present("regtest") {
            self.regtest(true);
        }
        if matches.is_present("rpcwait") {
            self.rpcwait(true);
        }
        if matches.is_present("rpcssl") {
            self.rpcssl(true);
        }
        if matches.is_present("rpcconnect") {
            self.rpcconnect(String::from(matches.value_of("rpcconnect").unwrap()));
        }
        if matches.is_present("rpcport") {
            self.rpcport(String::from(matches.value_of("rpcport").unwrap()));
        }
        if matches.is_present("rpcuser") {
            self.rpcuser(String::from(matches.value_of("rpcuser").unwrap()));
        }
        if matches.is_present("rpcpassword") {
            self.rpcpassword(String::from(matches.value_of("rpcpassword").unwrap()));
        }

        self
    }

    fn load_from_file(&mut self, filename: &str) -> &mut Self {
        let ini = ini::Ini::load_from_file(filename)
            .map_err(|e| self.file_error = e.msg)
            .unwrap_or(ini::Ini::new());

        if ini.sections().len() == 0 {
            return self;
        }

        let properties = ini.general_section();
        for (key, val) in properties.iter() {
            match key.as_ref() {
                "testnet" => {
                    self.testnet(val == "1");
                }
                "regtest" => {
                    self.regtest(val == "1");
                }
                "rpcwait" => {
                    self.rpcwait(val == "1");
                }
                "rpcssl" => {
                    self.rpcssl(val == "1");
                }
                "rpcconnect" => {
                    self.rpcconnect(val.to_owned());
                }
                "rpcport" => {
                    self.rpcport(val.to_owned());
                }
                "rpcuser" => {
                    self.rpcuser(val.to_owned());
                }
                "rpcpassword" => {
                    self.rpcpassword(val.to_owned());
                }
                _ => {}
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_file() {
        let config = ConfigBuilder::new().load_from_file("tests/zcash.conf").build();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.rpcconnect(), "127.0.0.1");
        assert_eq!(config.rpcport(), "18232");
        assert_eq!(config.rpcuser(), "regtest-user");
        assert_eq!(config.rpcpassword(),
                   "PpNWU2FiuUUFeMOT7opeylpwLEJKoI1SUwgPwIVwj7c=");
    }

    #[test]
    fn test_error_config_file() {
        let config = ConfigBuilder::new().load_from_file("fake_file.conf").build();
        assert!(config.is_err());
    }
}
