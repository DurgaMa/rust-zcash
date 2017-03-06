extern crate clap;

use clap::App;

fn main() {
    App::new("rust-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Zcash RPC Client")
        .get_matches();
}
