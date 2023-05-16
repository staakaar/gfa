use clap::Parser;
use rustyline::{Editor, DefaultEditor, error::ReadlineError};

#[derive(Parser)]
struct Curl {
    url: String,
    options: Vec<String>
}

const ICURL_INPUT_URL: &str = r#" \n URLを入力してください"#;

const INPUT_HTTP_METHOD: &str = r#" \n 利用するHTTPメソッドを選択してください。

    POST => 1
    PUT => 2
    DELETE => 3

    >> 
"#;

fn main()  {
    // icurlコマンド実行
    // url入力
    icurl_start();
    let mut url = input_url();
    // HTTPメソッドを選択させる
    let http_method = input_http();
    // オプション周りを標準出力する
    let curl_option = input_option();
    // POSTの場合はデータを入力させる
    // 最後にjpを利用して最終的に実行するcurlコマンドの内容を出力
    // 問題なければ実行　問題あれば　該当の箇所の編集ができるようにする

}

fn icurl_start() {
    println!("{ICURL_INPUT_URL}");
}

/// 
/// ユーザーにURLを入力してもらうメソッド
/// @return 
/// 
fn input_url() -> Result<(), std::io::Stderr> {
    let mut input = String::new();
    let url = std::io::stdin().read_line(&mut input);

    Ok(())
}

///
/// HTTPメソッドをユーザーに選択させる
///
fn input_http() -> Result<String, ReadlineError> {
    let mut rustyline = DefaultEditor::new()?;

    let readline = rustyline.readline(INPUT_HTTP_METHOD);
    let http_method = match readline {
        Ok(line) => {
            match line.trim() {
                "1" => "-X POST",
                "2" => "-X PUT",
                "3" => "-X DELETE",
            }
        },
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
        },
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    };

    Ok(http_method.to_owned())
}

fn input_option() -> Result<String, ReadlineError> {
    Ok(())
}