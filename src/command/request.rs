use super::curl_input::CurlInput;

pub struct Request {}

impl Request {

    pub async fn get(curl_input: &mut CurlInput) -> Result<(), reqwest::Error> {
        println!("{}", curl_input.port);
        let res = reqwest::get("test").await?;

        eprintln!("Response: {:?} {}", res.version(), res.status());
        eprintln!("Headers: {:#?}\n", res.headers());

        let body = res.text().await?;

        println!("{}", body);

        Ok(())
    }

    pub async fn post(curl_input: &mut CurlInput) -> Result<(), reqwest::Error> {
        println!("{}", curl_input.port);
        let res = reqwest::get("test").await?;
        let body = res.text().await?;
        Ok(())
    }

    pub async fn put(curl_input: &mut CurlInput) -> Result<(), reqwest::Error>  {
        println!("{}", curl_input.port);
        let res = reqwest::get("test").await?;
        let body = res.text().await?;
        Ok(())
    }

    pub async fn delete(curl_input: &mut CurlInput) -> Result<(), reqwest::Error>  {
        println!("{}", curl_input.port);
        let res = reqwest::get("test").await?;
        let body = res.text().await?;
        Ok(())
    }

}