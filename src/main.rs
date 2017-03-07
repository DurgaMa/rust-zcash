#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    App::from_yaml(yaml)
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();
}
