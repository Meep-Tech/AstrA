use super::{error::Error, error_builder::ErrorBuilder, token::Token, token_builder::TokenBuilder};

pub enum End {
    Match(TokenBuilder),
    Fail(ErrorBuilder),
    None,
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
    pub fn Error_In_Child(parent: TokenBuilder, err: Option<Error>) -> End {
        Error::in_child(parent, err)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Prop(parent: TokenBuilder, key: &str, err: Option<Error>) -> End {
        Error::in_prop(parent, key, err)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Variant(parent: TokenBuilder, err: Option<Error>) -> End {
        match err {
            Some(err) => {
                let mut error = err.to_builder();
                match parent.name {
                    Some(name) => {
                        error = error.tag(&name);
                    }
                    None => return End::Fail(error),
                }

                return End::Fail(error);
            }
            None => return End::None,
        }
    }

    #[allow(non_snake_case)]
    pub fn Unexpected(key: &str, found: &str) -> End {
        Error::unexpected(key, found)
    }

    #[allow(non_snake_case)]
    pub fn Missing(key: &str, expected: &str, found: &str) -> End {
        Error::missing(key, expected, found)
    }

    #[allow(non_snake_case)]
    pub fn Mismatch(key: &str, expected: &str, found: &str) -> End {
        Error::mismatch(key, expected, found)
    }
}
