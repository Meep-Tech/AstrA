use super::{error::Error, error_builder::ErrorBuilder, token::Token, token_builder::TokenBuilder};

pub enum End {
    Match(TokenBuilder),
    Fail(ErrorBuilder),
}

impl End {
    #[allow(non_snake_case)]
    pub fn Token() -> Option<End> {
        return Token::result();
    }

    #[allow(non_snake_case)]
    pub fn Skip() -> Option<End> {
        return None;
    }

    #[allow(non_snake_case)]
    pub fn Error(key: &str) -> Option<End> {
        return Some(End::Fail(Error::new(key)));
    }
}
