use crate::lexer::parsers::splayed;

splayed! {
    escape: [
        backtick_escape,
        newline_escape,
        quote_escape,
        tab_escape,
        escape_sequence,
    ]
}
