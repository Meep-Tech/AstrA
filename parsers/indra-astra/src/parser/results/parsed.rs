use super::{error::Error, token::Token};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Parsed {
    Pass(Token),
    Fail(Option<Error>),
}
