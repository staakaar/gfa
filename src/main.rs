use std::process::{self, Command};
use clap::Parser;

#[derive(Parser)]
struct Curl {
    url: String,
}

const ICURL_INPUT_URL: &str = r#" \n URLを入力してください"#;

fn main()  {
    // icurlコマンド実行
    // url入力
    icurl_start();
    input_url();
    // オプション周りを標準出力する
    // HTTPメソッドを選択させる
    // POSTの場合はデータを入力させる
    // 最後にjpを利用して最終的に実行するcurlコマンドの内容を出力
    // 問題なければ実行　問題あれば　該当の箇所の編集ができるようにする

}

fn icurl_start() {
    println!("{ICURL_INPUT_URL}");
}

fn input_url() -> Result<(), std::io::Stderr> {
    let mut input = String::new();
    let url = std::io::stdin().read_line(&mut input);

    Ok(())
}