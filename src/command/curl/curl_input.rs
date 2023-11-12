use std::collections::HashMap;

#[derive(Debug)]
pub struct CurlInput {
    protocol: String,
    host_name: String,
    port: String,
    http_method: String,
    authorization: String,
    query_params: HashMap<String, String>,
    body: String,
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