#![allow(incomplete_features)]
#![feature(lazy_cell)]
#![feature(const_hash)]
#![feature(type_name_of_val)]
#![feature(unsized_locals)]

pub mod highlighter;
pub mod lexer;
pub mod tests;
pub mod utils;

use std::collections::HashMap;

use lexer::cursor::Cursor;
use lexer::results::end::End;
use lexer::results::parsed::Parsed;
use lexer::results::token::Token;

use lexer::parsers::{self, name, named_entry};
use tests::lexer::parsers::test::{log_results, Outcome, Testable};

fn init() {
    parsers::init_all();
}

fn main() {
    init();
    let mut all_results: HashMap<String, Outcome> = HashMap::new();

    all_results.extend(named_entry::Parser::run_tests());
    all_results.extend(name::Parser::run_tests());

    log_results(&all_results);
}
