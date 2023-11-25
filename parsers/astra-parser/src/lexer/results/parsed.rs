use super::{error::Error, token::Token};

#[derive(PartialEq, Eq, Debug)]
pub enum Parsed {
    Token(Token),
    Error(Error),
}
