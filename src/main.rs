#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate env_logger;

mod rpc;

use clap::App;

fn main() {
    env_logger::init().unwrap();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    if matches.is_present("help") {
        let response = rpc::get_json_response("help");
        match response {
            Ok(json) => println!("{}", json.result),
            Err(err) => error!("{}", err),
        };
    }
}
