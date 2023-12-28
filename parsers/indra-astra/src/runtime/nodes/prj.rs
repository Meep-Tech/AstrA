use crate::{parser::results::r#match::Token, runtime::scope::Scope};

pub enum Analysis {
    Valid,
    Invalid,
}

#[allow(non_snake_case)]
pub fn Analyze(token: &Token, scope: Scope) -> Analysis {
    todo!()
}
