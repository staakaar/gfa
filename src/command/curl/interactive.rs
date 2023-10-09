use clap::Subcommand;
use inquire::{InquireError, Select};

use crate::common;

#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    Exec { command: Vec<String> }
}

impl Cmd {
    pub async fn run(self) {
        // HTTPメソッドの選択
        let ans: Result<&str, InquireError> = Select::new("Please select an HTTP method.", common::curl_config::get_http_method()).prompt();
        match ans {
            Ok(choice) => println!("{}! That's mine too!", choice),
            Err(_) => println!("There was an error, please try again"),
        }
    }
}