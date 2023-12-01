use super::{error::Error, token::Token};

#[derive(PartialEq, Eq, Debug)]
pub enum Parsed {
    Token(Token),
    Error(Error),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Optional {
    Token(Token),
    Error(Error),
    Ignored(Error),
}
