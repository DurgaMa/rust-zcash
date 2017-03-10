#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate reqwest;
extern crate hyper;

mod config;

use clap::{App, ArgMatches};
use config::Config;
use hyper::header::{Headers, Authorization, Basic};

fn main() {
    env_logger::init().unwrap();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    if matches.is_present("list") {
        println!("Show the list of commands!");
    } else {
        send_command(&matches);
    }
}

fn send_command(matches: &ArgMatches) {
    let config = Config::new(&matches);

    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
        username: config.rpcuser.to_string(),
        password: Some(config.rpcpassword.to_string()),
    }));

    let mut map = std::collections::HashMap::new();
    map.insert("method", matches.subcommand_name().unwrap());

    let client = reqwest::Client::new().unwrap();
    let res = client.post(&format!("http://{}:{}", config.rpcconnect, config.rpcport))
        .headers(headers)
        .json(&map)
        .send();

    match res {
        Ok(mut res) => {
            info!("Status: {}", res.status());
            info!("Headers:\n{}", res.headers());

            ::std::io::copy(&mut res, &mut ::std::io::stdout()).unwrap();
        }
        Err(res) => {
            error!("{}", res);
        }
    }
}
