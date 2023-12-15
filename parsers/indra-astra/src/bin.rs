use indra_astra::parser::{self};

fn init() {
    parser::init_all();
}

fn main() {
    init();
    // test_parsers(&[
    //     named_entry::Parser::Tests(),
    //     name::Parser::Tests(),
    //     tree::Parser::Tests(),
    //     tag::Parser::Tests(),
    // ]);
}
