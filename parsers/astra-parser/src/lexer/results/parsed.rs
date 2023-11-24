use super::{error::Error, token::Token};

#[derive(Debug)]
pub enum Parsed {
    Token(Token),
    Error(Error),
}
