use super::curl_input::CurlInput;

pub struct Request {}

impl Request {

    #[tokio::main]
    pub async fn get(curl_input: &mut CurlInput) -> Result<(), reqwest::Error> {
        println!("{}", curl_input.port);
        let res = reqwest::get("test").await?;

        eprintln!("Response: {:?} {}", res.version(), res.status());
        eprintln!("Headers: {:#?}\n", res.headers());

        let body = res.text().await?;

        println!("{}", body);

        Ok(())
    }

    #[tokio::main]
    pub async fn post(curl_input: &mut CurlInput) -> Result<(), reqwest::Error> {
        println!("{}", curl_input.port);
        let new_post = {};

        let new_post = reqwest::Client::new()
            .post("test")
            .json(&new_post)
            .send()
            .await?
            .json()
            .await?;

        Ok(())
    }

    #[tokio::main]
    pub async fn put(curl_input: &mut CurlInput) -> Result<(), reqwest::Error>  {
        println!("{}", curl_input.port);
        let res = reqwest::get("test").await?;
        let body = res.text().await?;
        Ok(())
    }

    #[tokio::main]
    pub async fn delete(curl_input: &mut CurlInput) -> Result<(), reqwest::Error>  {
        println!("{}", curl_input.port);
        let res = reqwest::get("test").await?;
        let body = res.text().await?;
        Ok(())
    }

}