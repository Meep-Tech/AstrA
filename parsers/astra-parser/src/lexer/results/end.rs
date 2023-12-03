use std::rc::Rc;

use crate::lexer::{cursor::Cursor, parser};

use super::{
    builder::Builder, error::Error, error_builder::ErrorBuilder, parsed::Parsed, token::Token,
    token_builder::TokenBuilder,
};

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
    pub fn Variant(parent: &str, result: Parsed) -> End {
        match result {
            Parsed::Pass(token) => End::Token_Variant(parent, token),
            Parsed::Fail(error) => End::Unexpected_Variant(parent, error),
        }
    }

    #[allow(non_snake_case)]
    pub fn Choice(
        parent: &str,
        cursor: &mut Cursor,
        options: &[&'static Rc<dyn parser::Parser>],
    ) -> End {
        let mut errors = Vec::new();
        for option in options {
            match option.parse_opt_at(cursor) {
                Parsed::Pass(token) => return Token::new().name(parent).child(token).end(),
                Parsed::Fail(err) => {
                    errors.push(err);
                }
            }
        }

        return End::Missing_One_Of(
            parent,
            options.iter().map(|option| option.get_name()).collect(),
            errors,
        );
    }

    #[allow(non_snake_case)]
    pub fn Missing_One_Of(parent: &str, options: Vec<&str>, failures: Vec<Option<Error>>) -> End {
        let mut error = Error::new("missing-choice-in-{}");
        error = error.text(&format!(
            "Required one of the following tokens in {}: \n{}",
            parent,
            options
                .iter()
                .map(|option| format!("\t- {}", option))
                .collect::<Vec<String>>()
                .join("\n")
        ));
        for failure in failures {
            error = error.child(Parsed::Fail(failure));
        }

        return End::Fail(error);
    }

    #[allow(non_snake_case)]
    pub fn Token_Variant(parent: &str, token: Token) -> End {
        let mut variant = token.to_builder();
        variant = variant.tag(parent);

        return End::Match(variant);
    }

    #[allow(non_snake_case)]
    pub fn Build_Token_For_Variant(parent: &str, builder: TokenBuilder) -> End {
        let mut variant = builder;
        variant = variant.tag(parent);

        return End::Match(variant);
    }

    #[allow(non_snake_case)]
    pub fn Build_Error_For_Variant(parent: &str, builder: ErrorBuilder) -> End {
        let mut error = builder;
        error = error.tag(parent);

        return End::Fail(error);
    }

    #[allow(non_snake_case)]
    pub fn Build_Token_For_Variant_Of_Type<T>(parent: &str) -> End
    where
        T: parser::Parser + 'static,
    {
        let mut variant = Token::of_type::<T>();
        variant = variant.tag(parent);

        return End::Match(variant);
    }

    #[allow(non_snake_case)]
    pub fn Build_Error_For_Variant_Of_Type<T>(parent: &str) -> End
    where
        T: parser::Parser + 'static,
    {
        let mut error = Error::new("unexpected-{}");
        error = error.text(&format!("Unexpected token in {}", parent));
        error = error.tag(parent);

        return End::Fail(error);
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
    pub fn Unexpected_Child(parent: TokenBuilder, err: Option<Error>) -> End {
        Error::in_child(parent, err)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Child(parent: TokenBuilder, err: Error) -> End {
        Error::in_child(parent, Some(err))
    }

    #[allow(non_snake_case)]
    pub fn Missing_Child(parent: TokenBuilder) -> End {
        Error::in_child(parent, None)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Prop(parent: TokenBuilder, key: &str, err: Option<Error>) -> End {
        Error::in_prop(parent, key, err)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Variant(parent: &str, err: Option<Error>) -> End {
        match err {
            Some(err) => {
                let mut error = err.to_builder();
                error = error.tag(parent);

                return End::Fail(error);
            }
            None => return End::None,
        }
    }

    #[allow(non_snake_case)]
    pub fn Unexpected_Variant(parent: &str, err: Option<Error>) -> End {
        match err {
            Some(err) => {
                let mut error = err.to_builder();
                error = error.tag(parent);

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
