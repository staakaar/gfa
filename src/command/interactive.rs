use clap::Subcommand;
use inquire::{InquireError, Select};
use crate::common::curl_config;

use crate::command::protocol::{HttpConn, File, Protocol, ProtocolType};

#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    Exec { command: Vec<String> }
}

impl Cmd {
    pub async fn run(self) {

        // プロトコルの指定
        let protocol_ans: Result<&str, InquireError> = Select::new("Please select an protocol name", curl_config::get_protocol()).prompt();
        
        let protocol_type: ProtocolType = match protocol_ans.unwrap() {
            "HTTP" => ProtocolType::HTTP,
            "FILE" => ProtocolType::FILE,
            _ => panic!("Please select an protocol name")
        };

        match protocol_type {
            ProtocolType::HTTP => HttpConn::exec(),
            ProtocolType::FILE => File::exec(),
        }

    }

}