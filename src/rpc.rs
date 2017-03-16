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

fn get_rpc_response(command: &str, user: &str, pw: &str) -> reqwest::Result<Response> {
    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
                                  username: user.to_owned(),
                                  password: Some(pw.to_owned()),
                              }));

    let mut map = ::std::collections::HashMap::new();
    map.insert("method", command);

    let client = Client::new().unwrap();
    client.post("http://127.0.0.1:18232")
        .headers(headers)
        .json(&map)
        .send()
}

pub fn get_json_response(command: &str, user: &str, pw: &str) -> reqwest::Result<JsonRpc> {
    get_rpc_response(command, user, pw)?.json()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_response_not_empty() {
        let json_response = get_json_response("help",
                                              "regtest-user",
                                              "PpNWU2FiuUUFeMOT7opeylpwLEJKoI1SUwgPwIVwj7c=");
        assert!(json_response.is_ok());

        let result = json_response.unwrap();
        assert!(result.error.is_empty());
        assert!(!result.result.is_empty());
    }
}
