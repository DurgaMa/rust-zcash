extern crate reqwest;
extern crate hyper;

use self::reqwest::{Response, Client};
use self::hyper::header::{Headers, Authorization, Basic};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpc {
    pub result: String,
    pub error: String,
    pub id: String,
}

fn get_rpc_response(command: &str) -> reqwest::Result<Response> {
    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
                                  username: "regtest-user".to_owned(),
                                  password: Some("PpNWU2FiuUUFeMOT7opeylpwLEJKoI1SUwgPwIVwj7c="
                                                     .to_owned()),
                              }));

    let mut map = ::std::collections::HashMap::new();
    map.insert("method", command);

    let client = Client::new().unwrap();
    client.post("http://127.0.0.1:18232")
        .headers(headers)
        .json(&map)
        .send()
}

pub fn get_json_response(command: &str) -> reqwest::Result<JsonRpc> {
    get_rpc_response(command)?.json()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_response_not_empty() {
        let json_response: reqwest::Result<JsonRpc> = get_json_response("help");
        assert!(json_response.is_ok());

        let result = json_response.unwrap();
        assert!(result.error.is_empty());
        assert!(!result.result.is_empty());
    }
}
