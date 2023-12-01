use super::{error::Error, error_builder::ErrorBuilder, token::Token, token_builder::TokenBuilder};

pub enum End {
    Match(TokenBuilder),
    Fail(ErrorBuilder),
}

impl End {
    #[allow(non_snake_case)]
    pub fn Token() -> End {
        Token::result()
    }

    #[allow(non_snake_case)]
    pub fn New() -> TokenBuilder {
        Token::new()
    }

    #[allow(non_snake_case)]
    pub fn Error(key: &str) -> End {
        End::Fail(Error::new(key))
    }

    #[allow(non_snake_case)]
    pub fn None() -> End {
        Error::none()
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Child(parent: TokenBuilder, err: Error) -> End {
        Error::in_child(parent, err)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Prop(parent: TokenBuilder, key: &str, err: Error) -> End {
        Error::in_prop(parent, key, err)
    }

    #[allow(non_snake_case)]
    pub fn Unexpected(key: &str, found: &str) -> End {
        Error::unexpected(key, found)
    }

    #[allow(non_snake_case)]
    pub fn Missing(key: &str, expected: &str, found: &str) -> End {
        Error::missing(key, expected, found)
    }
}
