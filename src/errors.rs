use crate::Token;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(pub &'static str, pub Option<Token>);
