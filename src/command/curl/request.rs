pub trait Request {
    fn get(&self);
    fn post(&self);
    fn put(&self);
    fn delete(&self);
}

impl Request {
    fn get(&self) {}

    fn post(&self) {}

    fn put(&self) {}

    fn delete(&self) {}
}