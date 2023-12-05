use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CurlInput {
    pub protocol: String,
    pub host_name: String,
    pub port: String,
    pub http_method: String,
    pub authorization: String,
    pub query_params: HashMap<String, String>,
    pub body: HashMap<String, String>,
}

impl CurlInput {
    // pub fn new() -> CurlInput {
    //     CurlInput {
    //         protocol: String::new(),
    //         host_name: String::new(),
    //         port: String::new(),
    //         http_method: String::new(),
    //         authorization: String::new(),
    //         query_params: HashMap::new(),
    //         body: HashMap::new(),
    //     }
    // }
}