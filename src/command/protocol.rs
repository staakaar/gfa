use std::cell::Cell;
use std::collections::HashMap;
use async_trait::async_trait;
use inquire::{InquireError, Select, Text};

use crate::command::authorization::{Authorization, Bearer};
use crate::command::headers::Header;
use crate::command::http::{HttpMethod, Get, Post, Put, Delete, Http};
use crate::command::params::Params;
use crate::command::curl_input::CurlInput;
use crate::command::request::Request;
use crate::common::curl_config;

#[async_trait]
pub trait Protocol {
    fn exec();
}

#[derive(Debug)]
pub enum ProtocolType {
    HTTP,
    FILE,
}

pub struct HttpConn {
    curl_option: Cell<CurlInput>
}

impl HttpConn {
    fn init() -> HttpConn {
        HttpConn { 
            curl_option: Cell::new(
                CurlInput { 
                    protocol: String::new(),
                    host_name: String::new(),
                    port: String::new(),
                    http_method: String::new(),
                    authorization: String::new(),
                    query_params: HashMap::new(),
                    body: HashMap::new(),
                }
            )
        } 
    }
}

impl Protocol for HttpConn {
    fn exec() {
        let mut curl_option: HttpConn = HttpConn::init();
        let curl_input: &mut CurlInput = curl_option.curl_option.get_mut();

        // ホスト名の選択
        let host_ans: Result<&str, InquireError> = Select::new("Please select an HOST name", curl_config::get_host()).prompt();

        let mut host_text: String = host_ans.unwrap().to_string();
        if host_text.eq("Other") {
            let host_text_ans: Result<String, InquireError> = Text::new("Please select an HOST name").prompt();
            match host_text_ans {
                Ok(_) => {
                    host_text = input_host_name();
                },
                Err(_) => println!("There was an error, please try again"),
            }
        }
        println!("{}", host_text);

        // ポート選択
        let port_ans: Result<String, InquireError> = Text::new("Please select an PORT name").prompt();
        let port = port_ans.unwrap();

        // HTTPメソッドの選択
        let http_ans: Result<&str, InquireError> = Select::new("Please select an HTTP method.", curl_config::get_http_method()).prompt();
        match http_ans {
            Ok(choice) => println!("{}! That's mine too!", choice),
            Err(_) => println!("There was an error, please try again"),
        }

        let http_type: HttpMethod = match http_ans.unwrap() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => panic!("Please select an protocol name")
        };

        match http_type {
            HttpMethod::GET => Get::exec(Get {}, curl_input),
            HttpMethod::POST => Post::exec(Post {}, curl_input),
            HttpMethod::PUT => Put::exec(Put {}, curl_input),
            HttpMethod::DELETE => Delete::exec(Delete {}, curl_input),
        };

        // params有無
        let is_params: Result<&str, InquireError> = Select::new("Set params?", curl_config::is_params()).prompt();
        match is_params.unwrap() {
            "YES" =>  Params::set(curl_input),
            "NO" => println!("OK"),
            _ => panic!("Please select yes or no"),
        }

        // header設定
        let is_header: Result<&str, InquireError> = Select::new("Set params?", curl_config::is_params()).prompt();
        match is_header.unwrap() {
            "YES" =>  Header::set(),
            "NO" => println!("OK"),
            _ => panic!("Please select yes or no"),
        }

        // body設定

        // Authorizationヘッダーの有無
        let authorization_token = Select::new("Do you specify an Authorization header?", curl_config::get_authorization()).prompt();
        match authorization_token.unwrap() {
            "Yes" =>  Authorization::select(&Bearer {}),
            "No" => println!("OK"),
            _ => panic!("Please select yes or no"),
        }

        println!("{:?}", http_type);
        // send request
        let _: Result<(), reqwest::Error> = match http_type {
            HttpMethod::GET => Request::get(curl_input),
            HttpMethod::POST => Request::post(curl_input),
            HttpMethod::PUT => Request::put(curl_input),
            HttpMethod::DELETE => Request::delete(curl_input),
            _ => panic!("test")
        };
    }
}

fn input_host_name() -> String {
    let host_name = Text::new("Please input Host name").prompt();
    match host_name {
        Ok(input) => return input,
        Err(_) => std::process::exit(1)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct File {}

impl Protocol for File {
    fn exec() {
        println!("fileが選択されました。")
    }
}