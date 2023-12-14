use std::rc::Rc;

use crate::parser::{self, cursor::Cursor};

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
    pub fn New() -> TokenBuilder {
        Token::New()
    }

    #[allow(non_snake_case)]
    pub fn Fail_As(key: &str) -> ErrorBuilder {
        Error::New(key)
    }

    #[allow(non_snake_case)]
    pub fn Token() -> End {
        Token::End()
    }

    #[allow(non_snake_case)]
    pub fn As_Variant(parent: &str, result: Parsed) -> End {
        match result {
            Parsed::Pass(token) => End::Token_Variant(parent, token),
            Parsed::Fail(error) => End::Unexpected_Variant_Of(parent, error),
        }
    }

    #[allow(non_snake_case)]
    pub fn New_Variant<TVariant>(parent: &str) -> End
    where
        TVariant: parser::Parser + 'static,
    {
        let mut variant = Token::Of_Type::<TVariant>();
        variant.add_tag(parent);

        return End::Match(variant);
    }

    #[allow(non_snake_case)]
    pub fn As<TVariant>(parent: &str, cursor: &mut Cursor) -> End
    where
        TVariant: parser::Parser + 'static,
    {
        End::As_Variant(parent, TVariant::Parse_At(cursor))
    }

    #[allow(non_snake_case)]
    pub fn Splay(
        parent: &str,
        cursor: &mut Cursor,
        variants: &[&'static Rc<dyn parser::Parser>],
    ) -> End {
        let mut errors = Vec::new();
        for option in variants {
            match option.parse_opt_at(cursor) {
                Parsed::Pass(token) => return token.to_builder().tag(parent).end(),
                Parsed::Fail(err) => {
                    errors.push(err);
                }
            }
        }

        return End::Missing_Variant(
            parent,
            variants.iter().map(|option| option.name()).collect(),
            errors,
        );
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
                Parsed::Pass(token) => return Token::New().name(parent).child(token).end(),
                Parsed::Fail(err) => {
                    errors.push(err);
                }
            }
        }

        return End::Missing_Choice(
            parent,
            options.iter().map(|option| option.name()).collect(),
            errors,
        );
    }

    #[allow(non_snake_case)]
    pub fn Child<TChild>(parent: &str, cursor: &mut Cursor) -> End
    where
        TChild: parser::Parser + 'static,
    {
        End::Child_Of::<TChild>(Token::With_Name(parent), cursor)
    }

    #[allow(non_snake_case)]
    pub fn Child_Of<TChild>(parent: TokenBuilder, cursor: &mut Cursor) -> End
    where
        TChild: parser::Parser + 'static,
    {
        match TChild::Parse_At(cursor) {
            Parsed::Pass(token) => parent.child(token).end(),
            Parsed::Fail(error) => End::Unexpected_Child_Of(parent, error),
        }
    }

    #[allow(non_snake_case)]
    pub fn Prop<TProp>(parent: &str, key: &str, cursor: &mut Cursor) -> End
    where
        TProp: parser::Parser + 'static,
    {
        End::Prop_Of::<TProp>(Token::With_Name(parent), key, cursor)
    }

    #[allow(non_snake_case)]
    pub fn Prop_Of<TProp>(parent: TokenBuilder, key: &str, cursor: &mut Cursor) -> End
    where
        TProp: parser::Parser + 'static,
    {
        match TProp::Parse_At(cursor) {
            Parsed::Pass(token) => parent.prop(key, token).end(),
            Parsed::Fail(error) => End::Error_In_Prop_Of(parent, key, error),
        }
    }

    #[allow(non_snake_case)]
    pub fn Token_Variant(parent: &str, token: Token) -> End {
        let mut variant = token.to_builder();
        variant.add_tag(&parent);

        return End::Match(variant);
    }

    #[allow(non_snake_case)]
    pub fn Build_Token_For_Variant(parent: &str, builder: TokenBuilder) -> End {
        let mut variant = builder;
        variant.add_tag(parent);

        return End::Match(variant);
    }

    #[allow(non_snake_case)]
    pub fn Build_Error_For_Variant(parent: &str, builder: ErrorBuilder) -> End {
        let mut error = builder;
        error.add_tag(parent);

        return End::Fail(error);
    }

    #[allow(non_snake_case)]
    pub fn Build_Token_For_Variant_Of_Type<T>(parent: &str) -> End
    where
        T: parser::Parser + 'static,
    {
        let mut variant = Token::Of_Type::<T>();
        variant.add_tag(parent);

        return End::Match(variant);
    }

    #[allow(non_snake_case)]
    pub fn TODO() -> End {
        Error::New("not_implemented_{}").tag("TODO").end()
    }

    #[allow(non_snake_case)]
    pub fn Error(key: &str) -> End {
        End::Fail(Error::New(key))
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

    #[allow(non_snake_case)]
    pub fn Missing_Choice(parent: &str, options: Vec<&str>, failures: Vec<Option<Error>>) -> End {
        let mut error = Error::New(format!("missing_choice_in_{}", parent).as_str());
        error.set_text(&format!(
            "Required one of the following choices in '{}': \n{}",
            parent,
            options
                .iter()
                .map(|option| format!("\t- {}", option))
                .collect::<Vec<String>>()
                .join("\n")
        ));
        for failure in failures {
            error.add_child(Parsed::Fail(failure));
        }

        return End::Fail(error);
    }

    #[allow(non_snake_case)]
    pub fn Missing_Variant(parent: &str, options: Vec<&str>, failures: Vec<Option<Error>>) -> End {
        let mut error = Error::New(format!("missing_variant_of_{}", parent).as_str());
        error.set_text(&format!(
            "Required one of the following tokens as a variant of '{}': \n{}",
            parent,
            options
                .iter()
                .map(|option| format!("\t- {}", option))
                .collect::<Vec<String>>()
                .join("\n")
        ));
        for failure in failures {
            error.add_child(Parsed::Fail(failure));
        }

        return End::Fail(error);
    }

    #[allow(non_snake_case)]
    pub fn Unexpected_Child_Of(parent: TokenBuilder, err: Option<Error>) -> End {
        Error::in_child(parent, err)
    }

    #[allow(non_snake_case)]
    pub fn Unexpected_Child(parent: &str, err: Option<Error>) -> End {
        Error::in_child(Token::With_Name(parent), err)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Child_Of(parent: TokenBuilder, err: Error) -> End {
        Error::in_child(parent, Some(err))
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Child(parent: &str, err: Error) -> End {
        Error::in_child(Token::With_Name(parent), Some(err))
    }

    #[allow(non_snake_case)]
    pub fn Missing_Child_Of(parent: TokenBuilder) -> End {
        Error::in_child(parent, None)
    }

    #[allow(non_snake_case)]
    pub fn Missing_Child(parent: &str) -> End {
        Error::in_child(Token::With_Name(parent), None)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Prop_Of(parent: TokenBuilder, key: &str, err: Option<Error>) -> End {
        Error::in_prop(parent, key, err)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Prop(parent: &str, key: &str, err: Option<Error>) -> End {
        Error::in_prop(Token::With_Name(parent), key, err)
    }

    #[allow(non_snake_case)]
    pub fn Error_In_Variant_Of(parent: &str, err: Option<Error>) -> End {
        match err {
            Some(err) => {
                let mut error = err.to_builder();
                error.add_tag(parent);

                return End::Fail(error);
            }
            None => return End::None,
        }
    }

    #[allow(non_snake_case)]
    pub fn Unexpected_Variant_Of(parent: &str, err: Option<Error>) -> End {
        match err {
            Some(err) => {
                let mut error = err.to_builder();
                error.add_tag(parent);

                return End::Fail(error);
            }
            None => return End::None,
        }
    }
}
