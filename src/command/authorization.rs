use inquire::{Select};

use crate::common::curl_config;

pub trait Authorization {
    fn select(&self);
}

#[derive(PartialEq, Clone, Debug)]
pub enum AuthorizationType {
    BEARER(Bearer),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Bearer {}

impl dyn Authorization {}

impl Authorization for Bearer {
    fn select(&self) {
        // let env_var = env::vars();
        // let filter_env_vars: Vec<String> = env_var.into_iter().filter(|x| x.0.contains("")).map(|x| x.0).collect();
        // let select_var = Select::new("Please select an HTTP method.", filter_env_vars).prompt();
        let select_var = Select::new("Please select an HTTP method.", curl_config::get_authorization_type()).prompt();
        match select_var.unwrap() {
            "No Auth" => println!("test"),
            "API Key" => println!("test"),
            "Bearer Token" => println!("test"),
            "JWT Bearer" => println!("test"),
            _ => std::process::exit(1)
        }
    }
}
