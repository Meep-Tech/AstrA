// use crate::utils::log;

// use super::{
//     cursor::Cursor,
//     results::{end::End, parsed::Parsed, token::Token},
// };

// pub struct Options {
//     revert_on_fail: bool,
//     fail_quietly: bool,
// }

// pub trait Rule = Fn(&mut Cursor) -> End;

// impl dyn Rule {
//     fn parse(&self, input: &str) -> Parsed {
//         let mut cursor = Cursor::New(input);
//         self.read(&mut cursor)
//     }

//     fn read(&self, cursor: &mut Cursor) -> Parsed {
//         self.read_with_options(
//             cursor,
//             Options {
//                 fail_quietly: false,
//                 revert_on_fail: false,
//             },
//         )
//     }

//     fn parse_opt(&self, input: &str) -> Parsed {
//         self.parse_with_options(
//             input,
//             Options {
//                 fail_quietly: true,
//                 revert_on_fail: true,
//             },
//         )
//     }

//     fn read_opt(&self, cursor: &mut Cursor) -> Parsed {
//         self.read_with_options(
//             cursor,
//             Options {
//                 fail_quietly: true,
//                 revert_on_fail: true,
//             },
//         )
//     }

//     fn parse_opt_or_skip(&self, input: &str) -> Parsed {
//         self.parse_with_options(
//             input,
//             Options {
//                 fail_quietly: true,
//                 revert_on_fail: false,
//             },
//         )
//     }

//     fn read_opt_or_skip(&self, cursor: &mut Cursor) -> Parsed {
//         self.read_with_options(
//             cursor,
//             Options {
//                 fail_quietly: true,
//                 revert_on_fail: false,
//             },
//         )
//     }

//     fn try_read(&self, cursor: &mut Cursor) -> Option<Token> {
//         match self.read_opt(cursor) {
//             Parsed::Pass(token) => Some(token),
//             _ => None,
//         }
//     }

//     fn try_parse(&self, input: &str) -> Option<Token> {
//         match self.parse_opt(input) {
//             Parsed::Pass(token) => Some(token),
//             _ => None,
//         }
//     }

//     fn parse_with_options(&self, input: &str, options: Options) -> Parsed {
//         let mut cursor = Cursor::New(input);
//         self.read_with_options(&mut cursor, options)
//     }

//     fn read_with_options(&self, cursor: &mut Cursor, options: Options) -> Parsed {
//         let name = std::any::type_name::<Self>();
//         log::color!("PARSE", log::Color::Green);
//         log::push_unique!("PARSE");
//         log::push!(name);
//         log::push_div!(":", log::Color::Green);
//         log::info!(&[":START"], &format!("@ {}", cursor.pos));

//         let start = if &options.revert_on_fail {
//             cursor.save()
//         } else {
//             cursor.pos
//         };

//         let result = match self(cursor) {
//             End::Match(token) => {
//                 let token = token.assure_name(name).build(start, cursor.prev_pos());
//                 log::info!(
//                     &[":END", "MATCH"],
//                     &format!("@ {} => {:#?}", cursor.prev_pos(), token).color(log::Color::Green),
//                 );
//                 Parsed::Pass(token)
//             }
//             End::Fail(error) => {
//                 let error = error
//                     .tag(name())
//                     .assure_name(name())
//                     .build(start, cursor.prev_pos());

//                 if options.revert_on_fail {
//                     cursor.restore();
//                 }

//                 if options.fail_quietly {
//                     if log::IS_VV {
//                         log::info!(
//                             &[
//                                 ":END",
//                                 &"IGNORED"
//                                     .effect(log::Effect::Strikethrough)
//                                     .color(log::Color::BrightBlack)
//                             ],
//                             &format!("@ {} => {:#?}", cursor.prev_pos(), error)
//                                 .effect(log::Effect::Strikethrough)
//                                 .color(log::Color::BrightBlack),
//                         );
//                     } else {
//                         log::info!(
//                             &[
//                                 ":END",
//                                 &"IGNORED"
//                                     .color(log::Color::BrightBlack)
//                                     .effect(log::Effect::Strikethrough)
//                             ],
//                             &format!("@ {}", cursor.prev_pos()).color(log::Color::BrightBlack),
//                         );
//                     }
//                 } else {
//                     log::info!(
//                         &[":END", "FAIL"],
//                         &format!("@ {} => {:#?}", cursor.prev_pos(), error)
//                             .color(log::Color::Red)
//                             .effect(log::Effect::Underline),
//                     );
//                 }

//                 Parsed::Fail(error)
//             }
//             End::None => {
//                 if options.revert_on_fail {
//                     cursor.restore();
//                 }

//                 log::info!(
//                     &[":END", &"NONE".color(log::Color::BrightBlack)],
//                     &format!("@ {}", cursor.prev_pos())
//                 );
//                 Parsed::Fail(None)
//             }
//         };

//         log::pop!();
//         log::pop!();
//         log::pop_unique!("PARSE");

//         result
//     }
// }

// pub fn test() {
//     let mut cursor = Cursor::New("hello world");
//     let rule = |cursor: &mut Cursor| {
//         cursor.read();
//         End::Match(Token::New())
//     };
//     let result = rule.read(&mut cursor);
//     println!("{:#?}", result);
// }

// //     fn parse(&self, input: &str) -> Parsed {
// //         let mut cursor = Cursor::new(input);
// //         self.read(&mut cursor)
// //     }

// //     fn read(cursor: &mut Cursor) -> Parsed {
// //         read_with_options(
// //             cursor,
// //             Options {
// //                 ignore_on_fail: false,
// //                 optional: false,
// //             },
// //         )
// //     }

// // fn parse_opt(input: &str) -> Parsed {
// //     parse_with_options(
// //         input,
// //         Options {
// //             ignore_on_fail: true,
// //             optional: true,
// //         },
// //     )
// // }

// // fn read_opt(cursor: &mut Cursor) -> Parsed {
// //     read_with_options(
// //         cursor,
// //         Options {
// //             ignore_on_fail: true,
// //             optional: true,
// //         },
// //     )
// // }

// // fn parse_opt_or_skip(input: &str) -> Parsed {
// //     parse_with_options(
// //         input,
// //         Options {
// //             ignore_on_fail: true,
// //             optional: false,
// //         },
// //     )
// // }

// // fn read_opt_or_skip(cursor: &mut Cursor) -> Parsed {
// //     read_with_options(
// //         cursor,
// //         Options {
// //             ignore_on_fail: true,
// //             optional: false,
// //         },
// //     )
// // }

// // fn try_read(cursor: &mut Cursor) -> Option<Token> {
// //     match read_opt(cursor) {
// //         Parsed::Pass(token) => Some(token),
// //         _ => None,
// //     }
// // }

// // fn try_parse(input: &str) -> Option<Token> {
// //     match parse_opt(input) {
// //         Parsed::Pass(token) => Some(token),
// //         _ => None,
// //     }
// // }

// // struct Options {
// //     optional: bool,
// //     ignore_on_fail: bool,
// // }

// // fn parse_with_options<TRule>(rule: TRule, input: &str, options: Options) -> Parsed
// // where
// //     TRule: Rule,
// // {
// //     let mut cursor = Cursor::new(input);
// //     read_with_options(rule, &mut cursor, options)
// // }

// // fn read_with_options<TRule>(rule: TRule, cursor: &mut Cursor, options: Options) -> Parsed
// // where
// //     TRule: Rule,
// // {
// //     let name = TRule::name();
// //     log::color!("PARSE", log::Color::Green);
// //     log::push_unique!("PARSE");
// //     log::push!(name());
// //     log::push_div!(":", log::Color::Green);
// //     log::info!(&[":START"], &format!("@ {}", cursor.pos));

// //     let start = if &options.optional {
// //         cursor.save()
// //     } else {
// //         cursor.pos
// //     };

// //     let result = match rule(cursor) {
// //         End::Match(token) => {
// //             let token = token.assure_name(name()).build(start, cursor.prev_pos());
// //             log::info!(
// //                 &[":END", "MATCH"],
// //                 &format!("@ {} => {:#?}", cursor.prev_pos(), token).color(log::Color::Green),
// //             );
// //             Parsed::Pass(token)
// //         }
// //         End::Fail(error) => {
// //             let error = error
// //                 .tag(name())
// //                 .assure_name(name())
// //                 .build(start, cursor.prev_pos());

// //             if optional {
// //                 cursor.restore();
// //             }

// //             if ignored {
// //                 if log::IS_VV {
// //                     log::info!(
// //                         &[
// //                             ":END",
// //                             &"IGNORED"
// //                                 .effect(log::Effect::Strikethrough)
// //                                 .color(log::Color::BrightBlack)
// //                         ],
// //                         &format!("@ {} => {:#?}", cursor.prev_pos(), error)
// //                             .effect(log::Effect::Strikethrough)
// //                             .color(log::Color::BrightBlack),
// //                     );
// //                 } else {
// //                     log::info!(
// //                         &[
// //                             ":END",
// //                             &"IGNORED"
// //                                 .color(log::Color::BrightBlack)
// //                                 .effect(log::Effect::Strikethrough)
// //                         ],
// //                         &format!("@ {}", cursor.prev_pos()).color(log::Color::BrightBlack),
// //                     );
// //                 }
// //             } else {
// //                 log::info!(
// //                     &[":END", "FAIL"],
// //                     &format!("@ {} => {:#?}", cursor.prev_pos(), error)
// //                         .color(log::Color::Red)
// //                         .effect(log::Effect::Underline),
// //                 );
// //             }

// //             Parsed::Fail(error)
// //         }
// //         End::None => {
// //             if optional {
// //                 cursor.restore();
// //             }

// //             log::info!(
// //                 &[":END", &"NONE".color(log::Color::BrightBlack)],
// //                 &format!("@ {}", cursor.prev_pos())
// //             );
// //             Parsed::Fail(None)
// //         }
// //     };

// //     log::pop!();
// //     log::pop!();
// //     log::pop_unique!("PARSE");

// //     result
// // }
