// use crate::lexer::{
//     cursor::Cursor,
//     parser,
//     results::{builder::Builder, end::End, parsed::Parsed},
// };

// pub struct Parser {}
// impl parser::Parser for Parser {
//     fn get_name(&self) -> &'static str {
//         return "struct";
//     }

//     fn rule(&self, cursor: &mut Cursor) -> End {
//         let result = tree::Parser::Parse_At(cursor);
//         match result {
//             Parsed::Token(token) => {
//                 return token.to_builder().end();
//             }
//             Parsed::Error(error) => {
//                 return error.to_builder().end();
//             }
//         }
//     }
// }

// pub mod tree {
//     use crate::lexer::{
//         cursor::Cursor,
//         parser,
//         parsers::indent::{self, Indents},
//         results::{end::End, token::Token},
//     };

//     use super::branch;

//     pub struct Parser {}
//     impl parser::Parser for Parser {
//         fn get_name(&self) -> &'static str {
//             return "tree";
//         }

//         fn rule(&self, cursor: &mut Cursor) -> End {
//             let mut result = Token::new();
//             match indent::Parse_At(cursor) {
//                 Indents::Increase(token) => {
//                     result = result.child(token);
//                 }
//                 Indents::Decrease(_) => {
//                     return End::Unexpected("initial-dedent", &cursor.curr_str())
//                 }
//                 Indents::Current(_) => {
//                     return End::Unexpected("initial-samedent", &cursor.curr_str())
//                 }
//                 _ => {}
//             };

//             loop {
//                 match branch::Parser::Parse_At(cursor) {
//                     Parsed::Token(token) => {
//                         result = result.child(token);
//                     }
//                     Parsed::Error(error) => {
//                         return error.to_builder().end();
//                     }
//                 }
//             }
//         }
//     }
// }

// pub mod branch {

//     use crate::lexer::{
//         cursor::Cursor,
//         parser,
//         parsers::indent::{self, Indents},
//         results::{end::End, token::Token},
//     };

//     pub struct Parser {}
//     impl parser::Parser for Parser {}
// }
