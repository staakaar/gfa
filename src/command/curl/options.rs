use std::collections::HashMap;

struct CurlOption<'a> {
    protocol: &'a str,
    host_name: String,
    port: String,
    http_method: &'a str,
    authorization: String,
    query_params: HashMap<&'a str, &'a str>,
    body: String,
}


impl CurlOption<'_> {
    pub fn new(&self) -> Self {
        Self {
            protocol: "",
            host_name: String::new(),
            port: String::new(),
            http_method: "",
            authorization: String::new(),
            query_params: HashMap::new(),
            body: String::new(),
        }
    }
}