use std::collections::HashMap;
use std::env;
use clap::Subcommand;
use inquire::{InquireError, Select, Text};

use crate::common;

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
        
        // ホスト名の選択
        let host_ans: Result<&str, InquireError> = Select::new("Please select an HOST name", common::curl_config::get_host()).prompt();
        if host_ans.unwrap() == "Other" {
            let host_text: Result<String, InquireError> = Text::new("Please select an HOST name").prompt();
            match host_text {
                Ok(_) => input_host_name(),
                Err(_) => println!("There was an error, please try again"),
            }
        }
        
        // ポート選択
        let port_ans: Result<String, InquireError> = Text::new("Please select an PORT name").prompt();
        
        // HTTPメソッドの選択
        let ans: Result<&str, InquireError> = Select::new("Please select an HTTP method.", common::curl_config::get_http_method()).prompt();
        match ans {
            Ok(choice) => println!("{}! That's mine too!", choice),
            Err(_) => println!("There was an error, please try again"),
        }

        // Authorizationヘッダーの登録
        let authorization_token = Select::new("Do you specify an Authorization header?", common::curl_config::get_authorization()).prompt();
        // 認証情報を .envに定義する or 環境変数を読み込む 認証情報のみをコピペさせて入力したほうがよさそう
        let env_var = env::vars();
        let filter_env_vars: Vec<String> = env_var.into_iter().filter(|x| x.0.contains("")).map(|x| x.0).collect();
        let select_var = Select::new("Please select an HTTP method.", filter_env_vars).prompt();

        // postメソッドの場合はbody入力
        // fileデータを添付する場合

    }

}

fn input_host_name() {
    let host_name = Text::new("Please input Host name").prompt();
}
