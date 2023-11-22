use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CurlInput {
    pub protocol: String,
    pub host_name: String,
    pub port: String,
    pub http_method: String,
    pub authorization: String,
    pub query_params: HashMap<String, String>,
    pub body: String,
}

impl CurlInput {
    pub fn new() -> CurlInput {
        CurlInput {
            protocol: String::new(),
            host_name: String::new(),
            port: String::new(),
            http_method: String::new(),
            authorization: String::new(),
            query_params: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn set_host_name(curl_option: &mut CurlInput, host: String) -> &mut CurlInput {
        curl_option.host_name = host;
        return curl_option;
    }

    pub fn set_port(curl_option: &mut CurlInput, port: String) -> &mut CurlInput {
        curl_option.port = port;
        return curl_option;
    }
}