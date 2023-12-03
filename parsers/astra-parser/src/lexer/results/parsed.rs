use super::{error::Error, token::Token};

#[derive(PartialEq, Eq, Debug)]
pub enum Parsed {
    Pass(Token),
    Fail(Option<Error>),
}
