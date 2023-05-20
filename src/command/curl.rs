use std::collections::HashMap;

use eyre::Result;
use clap::Parser;

/// Struct
#[derive(Parser)]
pub struct Curl {
    url: &'static str,
    options: Vec<String>,
    query_params: Vec<HashMap<String, String>>,
    // ユーザーの入力によって型がさまざまあるためプログラム上でjsonへパースする
    json_param: String,
}

/// Url
#[derive(Parser)]
pub struct Url {
    Scheme: String,
    SubDomain: String,
    Domain: String,
    Port: String,
    /// Endpoint Root
    Root: String,
}


/// const
const ICURL_INPUT_URL: &str = r#" \n URLを入力してください"#;

const INPUT_HTTP_METHOD: &str = r#" \n 利用するHTTPメソッドを選択してください。

    POST => 1
    PUT => 2
    DELETE => 3

    >> 
"#;

const INPUT_CURL_OPTION: &str = r#" \n 利用するオプションを選択してください。"#;



/// Enum

/// Method
#[derive(PartialEq, Clone, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}
/// パラメーターの種類
#[derive(PartialEq, Clone, Debug)]
pub enum ParamKind {
    Json,
    File,
    UrlEncode,
}

/// Impl
impl Curl {
    fn new(self) -> Self {
        Self {
            url,
            options,
            query_params,
            json_param,
        }
    }

    pub fn interactive(self) -> Result<()> {
        /// curlコマンドに必要な値をインタラクティブに入力していく
        Ok(())
    }

    fn build_command(self) -> Result<()> {}

    fn icurl_start() {
        println!("{ICURL_INPUT_URL}");
    }
    
    /// 
    /// ユーザーにURLを入力してもらうメソッド
    /// @return 
    /// 
    fn input_url() -> Result<usize, std::io::Stderr> {
        let mut input = String::new();
        let url = std::io::stdin().read_line(&mut input);
    
        println!("入力したURL: {:?}", url);
    
        Ok(url.unwrap())
    }
    
    ///
    /// HTTPメソッドをユーザーに選択させる
    ///
    fn input_http() -> Result<String, ReadlineError> {
        let mut rustyline = DefaultEditor::new()?;
    
        let readline = rustyline.readline(INPUT_HTTP_METHOD);
        let http_method = match readline {
            Ok(readline) => {
                match readline.trim() {
                    "1" => "-X POST",
                    "2" => "-X PUT",
                    "3" => "-X DELETE",
                    _ => process::exit(1)
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                process::exit(1);
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                process::exit(1);
            },
            Err(err) => {
                println!("Error: {:?}", err);
                process::exit(1);
            }
        };
    
        Ok(http_method.to_owned())
    }
    
    ///
    /// curlコマンドのオプション
    /// 
    fn input_option() -> Result<Vec<String>, ReadlineError> {
        let mut rustyline = DefaultEditor::new()?;
        let mut options: Vec<String> = vec![];
    
        loop {
            let readline = rustyline.readline(INPUT_CURL_OPTION);
    
            match readline {
                Ok(readline) => {
                    options.push(readline);
                    break;
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-Cが入力されたためプログラムを終了しました。");
                    break;
                }
                Err(_) => {
                    println!("エラー");
                    break;
                }
            };
        }
    
        Ok(options)
    }
    
    ///
    /// パラメーターの種類を選択させる
    /// 
    pub fn select_parameter_kind() {}
}