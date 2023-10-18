pub trait Protocol {
    fn exec(&self);
}

#[derive(PartialEq, Clone, Debug)]
pub enum Protocol {
    HTTP(Http),
    FILE(File),
}

pub struct Http {}

impl Protocol for Http {
    fn exec(&self) {
        println!("httpが選択されました。")
    }
}