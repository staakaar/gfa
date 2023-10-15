

pub trait Http {
    fn exec(&self);
}

#[derive(PartialEq, Clone, Debug)]
pub enum HttpMethod {
    GET(Get),
    POST(Post),
    PUT(Put),
    DELETE(Delete),
}

struct Get {}
impl Http for Get {
    fn exec(&self) {}
}

struct Post {}
impl Http for Post {
    fn exec(&self) {}
}
