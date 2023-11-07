use std::collections::HashMap;
use inquire::{InquireError, Text, required};
use serde_json::{self, json};

pub trait Http {
    fn exec(&self);
}

#[derive(PartialEq, Clone, Debug)]
pub enum HttpMethod {
    GET(Get),
    POST(Post),
    PUT(Put),
    DELETE(Delete),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Get {}

impl Http for Get {
    fn exec(&self) {
        println!("http.rs GET method")
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Post {}

impl Http for Post {
    fn exec(&self) {
        println!("http.rs POST method");
        // postメソッドの場合はbody入力
        let mut body_map: HashMap<String, String> = HashMap::new();

        let payload_text: Result<String, InquireError>= Text::new("Please input an Payload key").with_validator(required!()).prompt();

        let payload_list: Vec<&str> = match &payload_text {
            Ok(text) => {
                text.split_whitespace().collect::<Vec<&str>>()
            }
            Err(_) => {
                std::process::exit(1);
            }
        };

        for i in 0..payload_list.len() / 2 {
            let key = payload_list[i * 2].to_string();
            let value = payload_list[i * 2 + 1].to_string();

            body_map.insert(key, value);
        }

        // payloadを作成していく
        let payload = serde_json::to_string(&body_map).unwrap();
        let payload_json = json!(payload);

        let json_string = serde_json::to_string_pretty(&payload_json).unwrap();
        print!("{}", json_string);
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Put {}

impl Http for Put {
    fn exec(&self) {
        let mut body_map: HashMap<String, String> = HashMap::new();

        let payload_text: Result<String, InquireError>= Text::new("Please input an Payload key").with_validator(required!()).prompt();

        let payload_list: Vec<&str> = match &payload_text {
            Ok(text) => {
                text.split_whitespace().collect::<Vec<&str>>()
            }
            Err(_) => {
                std::process::exit(1);
            }
        };

        for i in 0..payload_list.len() / 2 {
            let key = payload_list[i * 2].to_string();
            let value = payload_list[i * 2 + 1].to_string();

            body_map.insert(key, value);
        }
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Delete {}

impl Http for Delete {
    fn exec(&self) {
        println!("http.rs Delete method")
    }
}