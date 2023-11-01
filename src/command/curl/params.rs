use std::collections::HashMap;

use inquire::{InquireError, Text};

pub struct Params<'a> {
    params: HashMap<&'a str, &'a str>
}

impl Params<'_> {
    pub fn set() {
        println!("setメソッド");
        let params: Result<String, InquireError> = Text::new("Please input query parameter").prompt();
    }
}