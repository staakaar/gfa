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
    pub async fn post() -> Result<(), reqwest::Error> {
        Ok(())
    }

    #[tokio::main]
    pub async fn put() -> Result<(), reqwest::Error>  {
        Ok(())
    }

    #[tokio::main]
    pub async fn delete() -> Result<(), reqwest::Error>  {
        Ok(())
    }
}