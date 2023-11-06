use std::collections::HashMap;

use inquire::{InquireError, Text};

pub struct Params<'a> {
    params: HashMap<&'a str, &'a str>
}

impl Params<'_> {
    pub fn set() {
        let params_input: Result<String, InquireError> = Text::new("Please input query parameter").prompt();

        let mut params_hash: HashMap<String, String> = HashMap::new();
        let params = params_input.unwrap();
        let params_list: Vec<&str> = params.trim().split_whitespace().collect();

        for i in 0..params_list.len() / 2 {
            let key = params_list[i * 2].to_string();
            let value = params_list[i * 2 + 1].to_string();

            params_hash.insert(key, value);
        }

    }
}