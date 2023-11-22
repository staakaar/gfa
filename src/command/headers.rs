use inquire::{InquireError, Text};

use crate::common::kvs::{self, Kvs};

pub struct Header {}

impl Header {
    pub fn set() {
        let params_input: Result<String, InquireError> = Text::new("Please input header").prompt();

        let params = params_input.unwrap();
        let params_list: Vec<&str> = params.trim().split_whitespace().collect();

        let kvs: Kvs = kvs::Kvs::new();
        kvs.set(params_list);
    }
}