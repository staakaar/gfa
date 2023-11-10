

pub trait Request {
    fn get(&self);
    fn post(&self);
    fn put(&self);
    fn delete(&self);
}

#[tokio::main]
impl Request {
    fn get(&self) {
        let res = reqwest::get().await?.json::<HashMap<String, String>>().await?;
    }

    fn post(&self) {}

    fn put(&self) {}

    fn delete(&self) {}
}