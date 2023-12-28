use crate::{parser::results::r#match::Token, runtime::scope::Scope};

use super::Any;

pub enum Analysis {
    Valid(Any),
    Invalid,
}

#[allow(non_snake_case)]
pub fn Analyze(token: &Token, scope: Scope) -> Analysis {
    todo!()
}
