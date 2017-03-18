extern crate reqwest;
extern crate hyper;

use self::reqwest::{Response, Client};
use self::hyper::header::{Headers, Authorization, Basic};
use config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpc {
    pub result: String,
    pub error: String,
    pub id: String,
}

fn get_rpc_response(command: &str,
                    connect: &str,
                    port: &str,
                    user: &str,
                    pw: &str)
                    -> reqwest::Result<Response> {
    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
                                  username: user.to_owned(),
                                  password: Some(pw.to_owned()),
                              }));

    let mut map = ::std::collections::HashMap::new();
    map.insert("method", command);

    let client = Client::new().unwrap();
    let address = format!("http://{}:{}", connect, port);
    client.post(&address)
        .headers(headers)
        .json(&map)
        .send()
}

fn get_json_response(command: &str,
                     connect: &str,
                     port: &str,
                     user: &str,
                     pw: &str)
                     -> reqwest::Result<JsonRpc> {
    get_rpc_response(command, connect, port, user, pw)?.json()
}

pub fn get_response(command: &str, config: &Config) -> reqwest::Result<JsonRpc> {
    get_json_response(command,
                      config.rpcconnect(),
                      config.rpcport(),
                      config.rpcuser(),
                      config.rpcpassword())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_response_not_empty() {
        let json_response = get_json_response("help",
                                              "127.0.0.1",
                                              "18232",
                                              "regtest-user",
                                              "PpNWU2FiuUUFeMOT7opeylpwLEJKoI1SUwgPwIVwj7c=");
        assert!(json_response.is_ok());

        let result = json_response.unwrap();
        assert!(result.error.is_empty());
        assert!(!result.result.is_empty());
    }
}
