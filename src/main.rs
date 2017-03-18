#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate ini;

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

    // TODO: Check if error
    let config = Config::load_from_args(&matches).unwrap();

    if let Some(subcommand) = matches.subcommand_name() {
        let response = rpc::get_response(subcommand, &config);
        match response {
            Ok(json) => println!("{}", json.result),
            Err(err) => error!("{}", err),
        };
    }
}
