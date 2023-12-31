// use crate::{
//     parser::{
//         results::{builder::Builder, parsed::Parsed, token::Token},
//         tokens::{
//             statement::{
//                 assignment::entry::named_entry,
//                 expression::{
//                     invocation::identifier::key::name,
//                     literal::{
//                         markup::element::text,
//                         structure::{self, tree},
//                     },
//                 },
//             },
//             symbol::operator::assigner::mutable_field_assigner,
//             whitespace::indent,
//         },
//     },
//     tests::parser::tokens::test::{Mockable, Test, TokenMocks},
// };

// use super::test::Testable;

// impl Testable for tree::Parser {
//     fn assure_tags(&self) -> Option<std::collections::HashSet<String>> {
//         let mut tags = std::collections::HashSet::new();
//         tags.insert(structure::KEY.to_string());

//         return Some(tags);
//     }

//     fn get_tests(&self) -> Vec<super::test::Test>
//     where
//         Self: 'static + Sized + crate::parser::Parser,
//     {
//         vec![
//             Test::tags::<Self>(
//                 &["One Named Entry"],
//                 "name: value",
//                 Parsed::Pass(
//                     Token::New()
//                         .name(tree::KEY)
//                         .child(Token::Mock::<indent::current::Parser>())
//                         .child(
//                             Token::New()
//                                 .name(named_entry::KEY)
//                                 .prop("key", Token::New().name(name::KEY).build(0, 3))
//                                 .prop(
//                                     "operator",
//                                     Token::New().name(mutable_field_assigner::KEY).build(4, 4),
//                                 )
//                                 .prop("value", Token::New().name(text::KEY).build(6, 10))
//                                 .build(0, 10),
//                         )
//                         .build(0, 10),
//                 ),
//             ),
//             Test::pattern_with_tags::<Self>(
//                 &["One Named Entry"],
//                 "{}",
//                 &[&named_entry::KEY],
//                 Parsed::Pass(
//                     Token::New()
//                         .name(tree::KEY)
//                         .child(Token::Mock::<indent::current::Parser>())
//                         .child(Token::Mock::<named_entry::Parser>())
//                         .mock(),
//                 ),
//             ),
//             Test::pattern_with_tags::<Self>(
//                 &["Two Named Entries"],
//                 "{}\n{}",
//                 &[&named_entry::KEY, &named_entry::KEY],
//                 Parsed::Pass(
//                     Token::New()
//                         .name(tree::KEY)
//                         .child(Token::Mock::<indent::current::Parser>())
//                         .child(Token::Mock::<named_entry::Parser>())
//                         .child(Token::Mock::<indent::current::Parser>())
//                         .child(Token::Mock::<named_entry::Parser>())
//                         .mock(),
//                 ),
//             ),
//             Test::pattern_with_tags::<Self>(
//                 &["Two Named Entries", "Empty Lines In Between"],
//                 "{}\n\n\n{}",
//                 &[&named_entry::KEY, &named_entry::KEY],
//                 Parsed::Pass(
//                     Token::New()
//                         .name(tree::KEY)
//                         .child(Token::Mock::<indent::current::Parser>())
//                         .child(Token::Mock::<named_entry::Parser>())
//                         .child(Token::Mock::<indent::current::Parser>())
//                         .child(Token::Mock::<named_entry::Parser>())
//                         .mock(),
//                 ),
//             ),
//         ]
//     }
// }
