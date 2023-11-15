use std::collections::HashMap;


pub struct Request {}

impl Request {

    #[tokio::main]
    pub async fn get() {
        let res = reqwest::get("test").await?.json::<HashMap<String, String>>().await?;
    }

    #[tokio::main]
    pub async fn post() {}

    #[tokio::main]
    pub async fn put() {}

    #[tokio::main]
    pub async fn delete() {}
}