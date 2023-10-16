

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

#[derive(PartialEq, Clone, Debug)]
pub struct Get {}

impl Http for Get {
    fn exec(&self) {
        println!("http.rs走っている")
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Post {}

impl Http for Post {
    fn exec(&self) {}
}


#[derive(PartialEq, Clone, Debug)]
pub struct Put {}

impl Http for Put {
    fn exec(&self) {}
}


#[derive(PartialEq, Clone, Debug)]
pub struct Delete {}

impl Http for Delete {
    fn exec(&self) {}
}