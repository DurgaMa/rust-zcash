#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

mod config;

use clap::App;
use config::Config;

fn main() {
    env_logger::init().unwrap();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    let _config = Config::new(&matches);
}
