// use crate::parser::{
//     results::{builder::Builder, error::Error, parsed::Parsed, token::Parser},
//     tokens::statement::{assignment::entry, expression::invocation::identifier::key::name},
// };

// use super::tests::Test;

// impl Testable for name::Parser {
//     fn assure_tags(&self) -> Option<std::collections::HashSet<String>> {
//         let mut tags = std::collections::HashSet::new();
//         tags.insert(entry::KEY.to_string());

//         return Some(tags);
//     }

//     fn get_tests(&self) -> Vec<Test> {
//         return vec![
//             Test::tags::<Self>(
//                 &["Alphabetic"],
//                 "abc",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 2)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphabetic", "Single Dash in Middle"],
//                 "abc-def",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 6)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphabetic", "Underscore", "Dash"],
//                 "abc_efg-hij",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 10)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphanumeric", "Numeric Start"],
//                 "123abc",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 5)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphanumeric", "Underscore"],
//                 "abc_123",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 6)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphanumeric", "Underscore", "Dash"],
//                 "abc_123-456",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 10)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphabetic", "Double Dash", "Error"],
//                 "abc--def",
//                 Parsed::Fail(Error::New("unexpected_repeat_lone_symbol_in_name").build(0, 3)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphabetic", "Double Underscore"],
//                 "abc__def",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 7)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphabetic", "Double Underscore", "Dash"],
//                 "abc__def-ghi",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 11)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphanumeric", "Numeric End"],
//                 "abc123",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 5)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphanumeric", "Numeric Middle"],
//                 "abc123def",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 8)),
//             ),
//             Test::tags::<Self>(
//                 &[
//                     "Alphanumeric",
//                     "Alphabetic Middle",
//                     "Numeric End",
//                     "Numeric Start",
//                 ],
//                 "123abc456",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 8)),
//             ),
//             Test::tags::<Self>(
//                 &[
//                     "Alphanumeric",
//                     "Alphabetic Middle",
//                     "Numeric End",
//                     "Numeric Start",
//                     "Underscore",
//                 ],
//                 "123abc_456",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 9)),
//             ),
//             Test::tags::<Self>(
//                 &[
//                     "Alphanumeric",
//                     "Alphabetic Middle",
//                     "Numeric End",
//                     "Numeric Start",
//                     "Underscore",
//                     "Dash",
//                 ],
//                 "123abc_456-789",
//                 Parsed::Pass(Parser::New().name(name::KEY).build(0, 13)),
//             ),
//             Test::tags::<Self>(
//                 &[
//                     "Alphanumeric",
//                     "Alphabetic Middle",
//                     "Numeric End",
//                     "Numeric Start",
//                     "Underscore",
//                     "Double Dash",
//                     "Error",
//                 ],
//                 "123abc_456--789",
//                 Parsed::Fail(Error::New("unexpected_repeat_lone_symbol_in_name").build(0, 10)),
//             ),
//             Test::tags::<Self>(
//                 &["Numeric", "Error"],
//                 "123",
//                 Parsed::Fail(Error::New("unexpected_pure_numeric_key_in_name").build(0, 2)),
//             ),
//             Test::tags::<Self>(
//                 &["Numeric Start", "Numeric End", "Underscore", "Error"],
//                 "123_123",
//                 Parsed::Fail(Error::New("unexpected_pure_numeric_key_in_name").build(0, 6)),
//             ),
//             Test::tags::<Self>(
//                 &["Numeric", "Underscore End", "Error"],
//                 "123_",
//                 Parsed::Fail(Error::New("unexpected_pure_numeric_key_in_name").build(0, 3)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphabetic", "Underscore End", "Error"],
//                 "abc_",
//                 Parsed::Fail(Error::New("unexpected_last_letter_in_name").build(0, 3)),
//             ),
//             Test::tags::<Self>(
//                 &["Alphabetic", "Underscore Start", "Error"],
//                 "_abc",
//                 Parsed::Fail(Error::New("unexpected_first_letter_in_name").build(0, 0)),
//             ),
//             Test::tags::<Self>(
//                 &["Dash Start", "Error"],
//                 "-abc",
//                 Parsed::Fail(Error::New("unexpected_first_letter_in_name").build(0, 0)),
//             ),
//             Test::tags::<Self>(
//                 &["Dash End", "Error"],
//                 "abc-",
//                 Parsed::Fail(Error::New("unexpected_last_letter_in_name").build(0, 3)),
//             ),
//         ];
//     }
// }
