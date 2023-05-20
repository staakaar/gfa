/// use
use std::process;

use clap::{Parser};
use command::ICurl;
use rustyline::{DefaultEditor, error::ReadlineError};
use eyre::Result;

/// mod
mod command;

#[derive(Parser)]
struct Curl {
    #[command(subcommand)]
    curl: ICurl,
}

impl Curl {
    fn run(self) -> Result<()> {
        self.curl.run()
    }
}

fn main() -> Result<()>  {
    // icurlコマンド実行
    Curl::parse().run()
    // url入力
    // icurl_start();
    // let url = input_url();
    // HTTPメソッドを選択させる
    // let http_method = input_http();
    // オプション周りを標準出力する
    // let curl_option = input_option();
    // パラメーターの種類を選択させる
    // let param_kind = select_parameter_kind();
    // クエリパラメーター

    // POSTの場合はデータを入力させる
    // 最後にjpを利用して最終的に実行するcurlコマンドの内容を出力
    // 問題なければ実行　問題あれば　該当の箇所の編集ができるようにする
    // Ok(())
}
