pub trait Protocol {
    fn exec(&self);
}

#[derive(PartialEq, Clone, Debug)]
pub enum ProtocolType {
    HTTP(HttpConn),
    FILE(File),
}

#[derive(PartialEq, Clone, Debug)]
pub struct HttpConn {}

impl Protocol for HttpConn {
    fn exec(&self) {
        println!("httpが選択されました。")
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct File {}

impl Protocol for File {
    fn exec(&self) {
        println!("fileが選択されました。")
    }
}