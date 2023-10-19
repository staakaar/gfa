use std::collections::HashMap;
use std::env;
use clap::Subcommand;
use inquire::{InquireError, Select, Text, required};
use serde_json::{self, json};
use crate::common;

use crate::command::curl::http::{Get, Post, Put, Delete, self};
use crate::command::curl::http::{Http, HttpMethod};
use crate::command::curl::protocol::{HttpConn, File, Protocol, ProtocolType};


struct CurlOption<'a> {
    protocol: &'a str,
    host_name: String,
    port: String,
    http_method: &'a str,
    authorization: String,
    query_params: HashMap<&'a str, &'a str>,
    body: String,
}

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

        // ホスト名の選択
        let host_ans: Result<&str, InquireError> = Select::new("Please select an HOST name", common::curl_config::get_host()).prompt();

        let mut host_text: String = "".to_owned();
        if host_ans.unwrap() == "Other" {
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
            HttpMethod::GET(Get) => Get.exec(),
            HttpMethod::POST(Post) => Post.exec(),
            HttpMethod::PUT(Put) => Put.exec(),
            HttpMethod::DELETE(Delete) => Delete.exec(),
        }

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
        
        let mut payload_enum = payload_list.iter().enumerate();
        while let Some((index, key)) = payload_enum.next() {
            if let Some((index,value)) = payload_enum.next() {
                body_map.insert(key.to_string(), value.to_string());
            }
        }

        // payloadを作成していく
        let payload = serde_json::to_string(&body_map).unwrap();
        let payload_json = json!(payload);

        // Authorizationヘッダーの登録
        let authorization_token = Select::new("Do you specify an Authorization header?", common::curl_config::get_authorization()).prompt();
        // 認証情報を .envに定義する or 環境変数を読み込む 認証情報のみをコピペさせて入力したほうがよさそう
        let env_var = env::vars();
        let filter_env_vars: Vec<String> = env_var.into_iter().filter(|x| x.0.contains("")).map(|x| x.0).collect();
        let select_var = Select::new("Please select an HTTP method.", filter_env_vars).prompt();
        

        // fileデータを添付する場合

    }

}

fn input_host_name() -> String {
    let host_name = Text::new("Please input Host name").prompt();
    match host_name {
        Ok(input) => return input,
        Err(_) => std::process::exit(1)
    }
}

impl CurlOption<'_> {
    pub fn new() {}
}