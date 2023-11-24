use inquire::{InquireError, Text, required};
use serde_json::{self};

use crate::common::kvs::{self, Kvs};

use super::curl_input::CurlInput;

pub trait Http {
    fn exec(self, curl_input: &mut CurlInput);
}

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct Get {}

impl Http for Get {
    fn exec(self, curl_input: &mut CurlInput) {
        print!("{}", curl_input.authorization);
        println!("http.rs GET method")
    }
}

pub struct Post {}

impl Http for Post {
    fn exec(self, curl_input: &mut CurlInput) {
        println!("http.rs POST method");

        let payload_text: Result<String, InquireError>= Text::new("Please input an Payload key").with_validator(required!()).prompt();

        let payload_list: Vec<&str> = match &payload_text {
            Ok(text) => {
                text.split_whitespace().collect::<Vec<&str>>()
            }
            Err(_) => {
                std::process::exit(1);
            }
        };

        let kvs: Kvs = kvs::Kvs::new();
        let kvs_hash: Kvs = kvs.set(payload_list);

        curl_input.body = kvs_hash.record;

        // payloadを作成していく
        // let payload = serde_json::to_string(&kvs.into()).unwrap();
        // let payload_json = json!(payload);

        // let json_string = serde_json::to_string_pretty(&payload_json).unwrap();
        print!("post exec end");
    }
}

pub struct Put {}

impl Http for Put {
    fn exec(self, curl_input: &mut CurlInput) {

        let payload_text: Result<String, InquireError>= Text::new("Please input an Payload key").with_validator(required!()).prompt();

        let payload_list: Vec<&str> = match &payload_text {
            Ok(text) => {
                text.split_whitespace().collect::<Vec<&str>>()
            }
            Err(_) => {
                std::process::exit(1);
            }
        };

        let kvs: Kvs = kvs::Kvs::new();
        kvs.set(payload_list);
    }
}

pub struct Delete {}

impl Http for Delete {
    fn exec(self, curl_input: &mut CurlInput) {

        let payload_text: Result<String, InquireError>= Text::new("Please input an Payload key").with_validator(required!()).prompt();

        let payload_list: Vec<&str> = match &payload_text {
            Ok(text) => {
                text.split_whitespace().collect::<Vec<&str>>()
            }
            Err(_) => {
                std::process::exit(1);
            }
        };

        let kvs: Kvs = kvs::Kvs::new();
        kvs.set(payload_list);
    }
}