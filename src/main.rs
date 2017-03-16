#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate derive_builder;

mod rpc;
mod config;

use clap::App;
use config::Config;

fn main() {
    env_logger::init().unwrap();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    let config = Config::load_config_from_args(&matches).unwrap();

    if let Some(subcommand) = matches.subcommand_name() {
        let response = rpc::get_json_response(subcommand, &config.rpcuser, &config.rpcpassword);
        match response {
            Ok(json) => println!("{}", json.result),
            Err(err) => error!("{}", err),
        };
    }
}
