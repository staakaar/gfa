use inquire::{InquireError, Text};

use crate::common::kvs::{self, Kvs};

use super::curl_input::CurlInput;

pub struct Params {}

impl Params {
    pub fn set(curl_input: &mut CurlInput) {
        let params_input: Result<String, InquireError> = Text::new("Please input query parameter").prompt();

        let params = params_input.unwrap();
        let params_list: Vec<&str> = params.trim().split_whitespace().collect();

        let kvs: Kvs = kvs::Kvs::new();
        let kvs_hash: Kvs = kvs.set(params_list);

        curl_input.query_params = kvs_hash.record;

    }
}