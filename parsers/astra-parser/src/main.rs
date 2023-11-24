pub mod lexer;

use lexer::cursor::Cursor;
use lexer::parsers::named_entry;
use lexer::results::end::End;

use lexer::results::parsed::Parsed;
use lexer::results::token::Token;

fn main() {
    let source = "hello: world";
    let result = named_entry::parse(source);
    println!("{:#?}", result);
}
