#![allow(unused_imports)]
use std::{any::TypeId, collections::HashMap, rc::Rc};

use crate::utils::ansi::{self, Styleable};

#[cfg(feature = "log")]
use crate::utils::ansi::Color;

use self::whitespace::indent;

pub mod attribute;
pub mod expression;
pub mod source;
pub mod statement;
pub mod symbol;
pub mod whitespace;

pub type Type = dyn super::Parser;

macro_rules! token {
    (
        $key:ident
        $(#$tags:ident)*
        =>
        $rule:expr
        $(,tests: $($tests:expr)*)?
        $(,subs: [$($subs:ident $(,)?)*])?
    ) => {
        pub const KEY: &str = stringify!($key);

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct Parser;
        impl crate::parser::Parser for Parser {
            #[allow(non_snake_case)]
            fn Get() -> Self {
                Self
            }

            fn get(&self) -> Box<dyn crate::parser::Parser> {
                Box::new(Self)
            }

            fn name(&self) -> &'static str {
                crate::parser::tokens::imports!();
                &KEY
            }

            fn subs(&self) -> Vec<Box<dyn crate::parser::Parser>> {
                crate::parser::tokens::imports!();
                {
                    let _empty = Vec::<Box::<dyn crate::parser::Parser>>::new();
                    _empty
                } $(;
                    vec![$(Box::new($subs::Parser::Get()),)*]
                )?
            }

            fn tags(&self) -> Vec<&'static str> {
                vec![$(stringify!($tags),)*]
            }

            fn rule(
                &self,
                cursor: &mut crate::parser::cursor::Cursor,
            ) -> crate::parser::results::end::End {
                crate::parser::tokens::imports!();
                let rule = |cursor: &mut Cursor| -> End { $rule(cursor) };
                rule(cursor)
            }

            $(fn get_tests(&self) -> Vec<crate::tests::parser::tokens::tests::Test> {
                crate::parser::tokens::imports!();
                use crate::tests::parser::tokens::tests::{pattern, unit, Mock, Mockable, Test};
                let tests: Vec<crate::tests::parser::tokens::tests::Test> = vec![$($tests,)*];

                tests
            })?
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
    ($key:ident: [$($parsers:ident $(,)?)*] subs: [$($subs:ident $(,)?)*]) => {
        $(pub mod $subs;)*

        crate::parser::tokens::splay! {
            $key: [$($parsers,)*]
            subs: [$($subs,)*]
        }
    };
}
pub(crate) use splay_mods;

macro_rules! splay {
    ($key:ident: [$($parsers:ident $(,)?)*]) => {
        crate::parser::tokens::splay!(
            $key: [$($parsers,)*]
            subs: [$($parsers,)*]
        );
    };
    (
        $key:ident: [$($parsers:ident $(,)?)*]
        subs: [$($subs:ident $(,)?)*]
    ) => {
        crate::parser::tokens::token! {
            $key => |cursor: &mut Cursor| {
                End::Splay(
                    &KEY,
                    cursor,
                    &[
                        $(
                            &$parsers::Parser::Get(),
                        )*
                    ]
                )
            },
            subs: [$($subs,)*]
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
                builder::Builder, end::End, error::Error, node::Node, parsed::Parsed, token::Token,
            },
            tokens::{self, source, statement, symbol, whitespace},
            Parser as _,
        };
    };
}
pub(crate) use imports;
// #endregion
