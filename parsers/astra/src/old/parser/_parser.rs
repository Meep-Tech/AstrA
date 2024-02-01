// use crate::parser::{cursor::Scanner, Reader};

// use super::{indents::Indents, Cursor, Token};

// pub enum Status {
//     Ok,
//     Err,
// }

// pub(crate) fn _parse_line_as_new_statement(
//     cursor: &mut Cursor,
//     source: &mut Token,
//     indent: Indents::Diff,
// ) -> Status {
//     let mut line = cursor.token().start();

//     macro_rules! eol {
//         () => {
//             source.push(line);
//             return Status::Ok;
//         };
//     }

//     match cursor.read() {
//         '/' => match cursor.read() {
//             // //line comment
//             '/' => {
//                 line.ttype = Token::Type::Comments::Line;
//                 // read as line comment
//                 todo!()
//             }
//             // /* block comment */
//             '*' => {
//                 line.ttype = Token::Type::Comments::Block;
//                 // read as block comment
//                 todo!()
//             }
//             _ => {
//                 // / division
//                 if indent.is_more() && cursor.prev_is_nbsp() {
//                     // read as division operation applied to the parent expression
//                     todo!()
//                 }
//                 // unexpected
//                 else {
//                     return cursor.err().unexpected_prev_in(&mut line, &["/", "*", "\\t", "' '"]);
//                 }
//             }
//         },
//         '#' => match cursor.read() {
//             '#' => match cursor.read() {
//                 // ### Region
//                 '#' => {
//                     line.ttype = Token::Type::Comments::Region;
//                     // read as doc region comment
//                     todo!()
//                 }
//                 _ => {
//                     // ## Doc
//                     if cursor.prev_is_nbsp() {
//                         line.ttype = Token::Type::Comments::Doc;
//                         // read as doc comment
//                         todo!()
//                     }
//                     // ##literal-tag
//                     else if !cursor.prev_is_nl() {
//                         line.ttype = Token::Type::Tags::Literal;
//                         // read as literal tag
//                         todo!()
//                     }
//                     // ##
//                     else {
//                         line.ttype = Token::Type::Comments::Doc;
//                         eol!();
//                     }
//                 }
//             },
//             _ => {
//                 // # unexpected
//                 if cursor.prev_is_nbsp() {
//                     // reserved syntax (comment or type casting potentially?)
//                     todo!()
//                 }
//                 // #Type
//                 else if !cursor.prev_is_nl() {
//                     line.ttype = Token::Type::Attributes::Tags::Own;
//                     // read as own tag
//                     todo!()
//                 } else {
//                     return cursor.err().unexpected_prev_in(&mut line, &["#", "\\S"]);
//                 }
//             }
//         },
//         '>' => match cursor.read() {
//             '>' => match cursor.read() {
//                 // >>#OutputType
//                 '#' => {
//                     line.ttype = Token::Type::Tags::Output;
//                     // read as output tag
//                     todo!()
//                 }
//                 _ => {
//                     // >> Procedural child entry
//                     if indent.is_more() {
//                         line.ttype = Token::Type::Procedurals::Anonymous;
//                         // read as an anonymous procedural
//                         todo!()
//                     }
//                     // UNEXPECTED
//                     else {
//                         return cursor.err().unexpected_prev_in(&mut line, &["#", "\\s"]);
//                     }
//                 }
//             },
//             '#' => {
//                 // >#InputType
//                 line.ttype = Token::Type::Tags::Input;
//                 // read as input tag
//                 todo!()
//             }
//             _ => {
//                 // > initial-entry
//                 if indent.is_more() && cursor.prev_is_nbsp() {
//                     let line_prefix = cursor.token().at_prev(Token::Type::Modifiers::LinePrefix);

//                     line.set("line_prefix", line_prefix);

//                     // read as entry
//                     todo!()
//                 }
//                 // >input|als#type
//                 else {
//                     line.ttype = Token::Type::Attributes::Input;
//                     // read as an input
//                     todo!()
//                 }
//             }
//         },
//         '|' => match cursor.read() {
//             // |>input-alias
//             '>' => {
//                 line.ttype = Token::Type::Attributes::Aliases::Input;
//                 // read as an input-only alias
//                 todo!()
//             }
//             _ => {
//                 // |alias
//                 line.ttype = Token::Type::Attributes::Aliases::Own;
//                 // read as an alias
//                 todo!()
//             }
//         },
//         '<' => todo!("Generic Attribute, Entry, or Deconstruction"),
//         '-' => todo!("Ordered Entry, Indented Flag, or Indented Subtraction"),
//         '.' => todo!("Local Line Prefix or Dot Lookup"),
//         '_' => todo!("Anonymous Entry (_), Access-Limited Variable Key (_key), or Access-Limited Input Prefix (_>)"),
//         '{' => todo!("Map Structure"),
//         '[' => todo!("Array Structure"),
//         '(' => todo!("Group Structure"),
//         ':' => {
//             if indent.is_more()
//                 && source.is_in::<Token::Type::Entries>()
//                 && !source.has("operator")
//             {
//                 // read as an operator
//                 todo!()
//             } else {
//                 cursor.err().invalid_prev_in(source,
//                     &format!(
//                         "Entry already has an assignment operator on previous line: '{:?}'.",
//                         source.prop("operator").unwrap()
//                     )
//                 );
//             }
//         },
//         '@' | '$' => {
//             // $name or @name
//             line.ttype = Token::Type::Identifiers::Keys::Name;
//         },
//         _ => {
//             let c = cursor.prev();
//             if c.is_alphabetic() {

//             } else if c.is_numeric() {

//             } else {
//                 cursor.err().unexpected_prev_in(source, &["/", "#", ">", "|", ".", "<", "-", ":", "_", "@", "$", "\\w"]);
//             }
//         }
//     }

//     eol!();
// }
