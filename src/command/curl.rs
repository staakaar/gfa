use std::process;

use eyre::Result;
use clap::Subcommand;
use rustyline::{DefaultEditor, error::ReadlineError};

mod interactive;

/// Struct
#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    #[command(subcommand)]
    Interactive(interactive::Cmd)
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
impl Cmd {

    #[tokio::main(flavor = "current_thread")]
    pub async fn interactive(self) -> Result<()> {

        match self {
            Self::Interactive(interactive) => interactive.run().await,
        }
        
        // url入力
        Self::icurl_start();
        let url = Self::input_url();
        // HTTPメソッドを選択させる
        let http_method = Self::input_http();
        // オプション周りを標準出力する
        let curl_option = Self::input_option();
        // パラメーターの種類を選択させる
        let param_kind = Self::select_parameter_kind();
        Ok(())
    }

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