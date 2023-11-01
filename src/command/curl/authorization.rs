

pub trait Authorization {
    fn check(&self);
}

#[derive(PartialEq, Clone, Debug)]
pub enum AuthorizationType {
    BEARER(Bearer),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Bearer {}

impl Authorization for Bearer {
    fn check(&self) {}
}
