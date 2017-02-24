fn main() {
    if std::env::args().any(|a| a == "--version" || a == "-v") {
        println!("{}\n", version_message());
        return;
    }
}

fn version_message() -> String {
    format!("Zcash RPC client version v{}", env!("CARGO_PKG_VERSION"))
}
