#![allow(unused_imports)]
use std::{any::TypeId, collections::HashMap, rc::Rc};

use crate::utils::log::{self, Styleable};

#[cfg(feature = "log")]
use crate::utils::log::Color;

use self::{
    statement::expression::invocation::identifier::key::name,
    symbol::operator::assigner::mutable_field_assigner, whitespace::indent,
};

use super::Type;

pub mod attribute;
pub mod source;
pub mod statement;
pub mod symbol;
pub mod whitespace;

pub type TokenType = dyn Type + Sync + 'static;
pub type Instance<TParser = TokenType> = TParser;

macro_rules! token {
    ($key:ident => $rule:expr) => {
        pub const KEY: &str = stringify!($key);

        pub struct Token;
        impl crate::parser::Type for Token {
            fn name(&self) -> &'static str {
                crate::parser::tokens::imports!();
                &KEY
            }

            fn rule(
                &self,
                cursor: &mut crate::parser::cursor::Cursor,
            ) -> crate::parser::results::end::End {
                crate::parser::tokens::imports!();
                let rule = |cursor: &mut Cursor| -> End { $rule(cursor) };
                rule(cursor)
            }
        }
    };

    ($key:ident => $rule:expr, tests: $($tests:expr)*) => {
        pub const KEY: &str = stringify!($key);

        pub struct Token;
        impl crate::parser::Type for Token {
            fn name(&self) -> &'static str {
                crate::parser::tokens::imports!();
                &KEY
            }

            fn rule(
                &self,
                cursor: &mut crate::parser::cursor::Cursor,
            ) -> crate::parser::results::end::End {
                crate::parser::tokens::imports!();
                let rule = |cursor: &mut Cursor| -> End { $rule(cursor) };
                rule(cursor)
            }

            fn get_tests(&self) -> Vec<crate::tests::parser::tokens::tests::Test> {
                crate::parser::tokens::imports!();
                use crate::tests::parser::tokens::tests::{pattern, unit, Mock, Mockable, Test};
                let tests: Vec<crate::tests::parser::tokens::tests::Test> = vec![$($tests,)*];

                tests
            }
        }
    };
}
pub(crate) use token;

macro_rules! splay_mods {
    ($key:ident: [$($parsers:ident $(,)?)*]) => {
        $(pub mod $parsers;)*

        crate::parser::tokens::splay! {
            $key: [$($parsers,)*]
        }
    };

    (#testable, $key:ident: [$($parsers:ident $(,)?)*]) => {

        crate::parser::tokens::splay! {
            #testable,
            $key: [$($parsers,)*]
        }
    };
}
pub(crate) use splay_mods;

macro_rules! splay {
    ($key:ident: [$($parsers:ident $(,)?)*]) => {
        crate::parser::tokens::token! {
            $key => |cursor: &mut Cursor| {
                End::Splay(
                    &KEY,
                    cursor,
                    &[
                        $(
                            &$parsers::Token::Get(),
                        )*
                    ]
                )
            }
        }
    };

    (#testable, $key:ident: [$($parsers:ident $(,)?)*]) => {
        crate::parser::tokens::token! {
            #testable,
            $key => |cursor: &mut Cursor| {
                End::Splay(
                    &KEY,
                    cursor,
                    &[
                        $(
                            &$parsers::Token::Get(),
                        )*
                    ]
                )
            }
        }
    };
}
pub(crate) use splay;

// #region Internal
macro_rules! imports {
    () => {
        use crate::parser::{
            context::{self, Context, Language},
            cursor::Cursor,
            fs,
            results::{
                builder::Builder, end::End, error::Error, node::Node, parsed::Parsed,
                r#match::Match, r#match::Token,
            },
            tokens::{self, source, statement, symbol, whitespace},
            Type as _,
        };
    };
}
pub(crate) use imports;
// #endregion
