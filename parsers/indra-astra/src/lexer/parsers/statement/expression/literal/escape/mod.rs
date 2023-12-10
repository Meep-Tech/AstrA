use crate::lexer::parsers::splay_mods;

splay_mods! {
    escape: [
        backtick_escape,
        newline_escape,
        quote_escape,
        tab_escape,
        escape_sequence,
    ]
}
