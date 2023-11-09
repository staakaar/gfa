use clap::Subcommand;
use inquire::{InquireError, Select};
use serde_json::{self, json};
use crate::common;

use crate::command::curl::http::{Get, Post, Put, Delete};
use crate::command::curl::http::{Http, HttpMethod};
use crate::command::curl::protocol::{HttpConn, File, Protocol, ProtocolType};
use crate::command::curl::params::Params;
use crate::command::curl::authorization::{Authorization, AuthorizationType, Bearer};

use super::headers::Header;

#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    Exec { command: Vec<String> }
}

impl Cmd {
    pub async fn run(self) {
        // プロトコルの指定
        let protocol_ans: Result<&str, InquireError> = Select::new("Please select an protocol name", common::curl_config::get_protocol()).prompt();
        
        let protocol_type: ProtocolType = match protocol_ans.unwrap() {
            "HTTP" => ProtocolType::HTTP(HttpConn {}),
            "FILE" => ProtocolType::FILE(File {}),
            _ => panic!("Please select an protocol name")
        };

        match protocol_type {
            ProtocolType::HTTP(http) => http.exec(),
            ProtocolType::FILE(file) => file.exec(),
        }
        
        // HTTPメソッドの選択
        let http_ans: Result<&str, InquireError> = Select::new("Please select an HTTP method.", common::curl_config::get_http_method()).prompt();
        match http_ans {
            Ok(choice) => println!("{}! That's mine too!", choice),
            Err(_) => println!("There was an error, please try again"),
        }

        let http_type: HttpMethod = match http_ans.unwrap() {
            "GET" => HttpMethod::GET(Get {}),
            "POST" => HttpMethod::POST(Post {}),
            "PUT" => HttpMethod::PUT(Put {}),
            "DELETE" => HttpMethod::DELETE(Delete {}),
            _ => panic!("Please select an protocol name")
        };

        match http_type {
            HttpMethod::GET(get) => get.exec(),
            HttpMethod::POST(post) => post.exec(),
            HttpMethod::PUT(put) => put.exec(),
            HttpMethod::DELETE(delete) => delete.exec(),
        }

        // params有無
        let is_params: Result<&str, InquireError> = Select::new("Set params?", common::curl_config::is_params()).prompt();
        match is_params.unwrap() {
            "YES" =>  Params::set(),
            "NO" => println!("OK"),
            _ => panic!("Please select yes or no"),
        }

        // header設定
        let is_header: Result<&str, InquireError> = Select::new("Set params?", common::curl_config::is_params()).prompt();
        match is_header.unwrap() {
            "YES" =>  Header::set(),
            "NO" => println!("OK"),
            _ => panic!("Please select yes or no"),
        }

        // body設定

        // Authorizationヘッダーの有無
        let authorization_token = Select::new("Do you specify an Authorization header?", common::curl_config::get_authorization()).prompt();
        match authorization_token.unwrap() {
            "YES" =>  Authorization::check(&Bearer {}),
            "NO" => println!("OK"),
            _ => panic!("Please select yes or no"),
        }

    }

}