use inquire::{InquireError, Select, Text};
use crate::common;

pub trait Protocol {
    fn exec(&self);
}

#[derive(PartialEq, Clone, Debug)]
pub enum ProtocolType {
    HTTP(HttpConn),
    FILE(File),
}

#[derive(PartialEq, Clone, Debug)]
pub struct HttpConn {}

impl Protocol for HttpConn {
    fn exec(&self) {
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
    }
}

fn input_host_name() -> String {
    let host_name = Text::new("Please input Host name").prompt();
    match host_name {
        Ok(input) => return input,
        Err(_) => std::process::exit(1)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct File {}

impl Protocol for File {
    fn exec(&self) {
        println!("fileが選択されました。")
    }
}