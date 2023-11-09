use std::collections::HashMap;

#[derive(Debug)]
pub struct CurlOption {
    protocol: String,
    host_name: String,
    port: String,
    http_method: String,
    authorization: String,
    query_params: HashMap<String, String>,
    body: String,
}


// pub trait CurlInput {
//     fn new() -> CurlOption;
// }

impl CurlOption {
    pub fn new() -> CurlOption {
        CurlOption {
            protocol: String::new(),
            host_name: String::new(),
            port: String::new(),
            http_method: String::new(),
            authorization: String::new(),
            query_params: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn set_host_name(curl_option: &mut CurlOption, host: String) -> &mut CurlOption {
        curl_option.host_name = host;
        return curl_option;
    }
}