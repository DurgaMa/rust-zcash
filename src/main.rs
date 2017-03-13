#[macro_use]
extern crate clap;

extern crate env_logger;

extern crate reqwest;
extern crate hyper;

use clap::App;

fn main() {
    env_logger::init().unwrap();
    let yaml = load_yaml!("cli.yml");
    App::from_yaml(yaml)
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{Response, Client};
    use hyper::header::{Headers, Authorization, Basic};

    fn get_rpc_response(command: &str) -> reqwest::Result<Response> {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
                                      username: "".to_owned(),
                                      password: Some("".to_owned()),
                                  }));

        let mut map = std::collections::HashMap::new();
        map.insert("method", command);

        let client = Client::new().unwrap();
        client.post("http://127.0.0.1:18232")
            .headers(headers)
            .json(&map)
            .send()
    }

    #[test]
    fn test_rpc_authentication() {
        let result = get_rpc_response("help");
        assert!(result.is_ok());
    }
}
