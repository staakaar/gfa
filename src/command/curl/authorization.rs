use std::env;
use inquire::{Select};

pub trait Authorization {
    fn check(&self);
}

#[derive(PartialEq, Clone, Debug)]
pub enum AuthorizationType {
    BEARER(Bearer),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Bearer {}

impl Authorization for Bearer {
    fn check(&self) {
        let env_var = env::vars();
        let filter_env_vars: Vec<String> = env_var.into_iter().filter(|x| x.0.contains("")).map(|x| x.0).collect();
        let select_var = Select::new("Please select an HTTP method.", filter_env_vars).prompt();
    }
}
