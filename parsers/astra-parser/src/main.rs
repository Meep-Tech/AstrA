pub mod lexer;
mod tests;

use lexer::cursor::Cursor;
use lexer::results::end::End;
use lexer::results::parsed::Parsed;
use lexer::results::token::Token;

use lexer::parsers::{self, named_entry};
use tests::lexer::parsers::test::Testable;

fn init() {
    parsers::initalize_all();
}

fn main() {
    init();
    named_entry::Parser::run_tests();
    //let source = "hello: world";
    //let result = named_entry::parse(source);
    //println!("{:#?}", result);
}
