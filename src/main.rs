use std::{io::BufReader, fs::File};
use clap::Parser;

#[derive(Parser)]
struct Curl {
    url: String,
}

fn main()  {
    // icurlコマンド実行
    // url入力
    // オプション周りを標準出力する
    // HTTPメソッドを選択させる
    // POSTの場合はデータを入力させる
    // 最後にjpを利用して最終的に実行するcurlコマンドの内容を出力
    // 問題なければ実行　問題あれば　該当の箇所の編集ができるようにする

}
