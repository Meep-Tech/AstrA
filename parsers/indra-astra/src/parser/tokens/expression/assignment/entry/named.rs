use crate::parser::{
    cursor::Cursor,
    results::{token::Token, token_builder::TokenBuilder},
    tokens::{
        attribute,
        expression::{self, identifier::key},
        expression::{identifier::key::name, literal::structure::tree},
        statement::branch,
        symbol::operator::assigner,
        token,
        whitespace::indent::{self, Indents},
    },
};

token! {
    named_entry => |cursor: &mut Cursor | {
        let mut result = Token::New();

        // pre-key attributes
        if let Some(preceeding_attributes) = attribute::group::Parser::Try_Parse_At(cursor) {
            result.add_child(preceeding_attributes);
        }

        // key
        let key = key::Parser::Parse_At(cursor);
        let mut indent_increased = false;
        match key {
            Parsed::Pass(key) => {
                result.set_prop("key", key);

                // post-key indent
                cursor.save();
                match indent::Parse_Opt_Or_Skip_At(cursor) {
                    Indents::Increase(token) => {
                        result.add_child(token);
                        indent_increased = true;
                    }
                    Indents::Decrease(_) => {
                        cursor.restore();
                        return result.end_at(cursor.prev_non_ws_pos()).end();
                    }
                    Indents::Current(_) => {
                        cursor.restore();
                        return result.end_at(cursor.prev_non_ws_pos()).end();
                    }
                    _ => {
                        cursor.skip_ws();
                    }
                }
                cursor.pop();

                // post-key attributes
                if cursor.prev_is_ws() {
                    if let Some(attrs) = attribute::trailing::Parser::Try_Parse_At(cursor) {
                        result.add_child(attrs);

                        // post-key attributes indent
                        cursor.save();
                        match indent::Parse_Opt_Or_Skip_At(cursor) {
                            Indents::Increase(token) => {
                                result.add_child(token);
                                indent_increased = true;
                            }
                            Indents::Decrease(_) => {
                                cursor.restore();
                                return result.end_at(cursor.prev_non_ws_pos()).end();

                            }
                            Indents::Current(_) => {
                                cursor.restore();
                                return result.end_at(cursor.prev_non_ws_pos()).end();
                            }
                            _ => {
                                cursor.skip_ws();
                            }
                        }
                        cursor.pop();
                    }
                }

                // operator
                let operator = assigner::Parser::Parse_At(cursor);
                match operator {
                    Parsed::Pass(operator) => {
                        result.set_prop("operator", operator);

                        // post-operator attributes
                        if let Some(attrs) = attribute::trailing::Parser::Try_Parse_At(cursor) {
                            result.add_child(attrs);
                        }

                        // post-operator indent
                        let base_indent = cursor.curr_indent();
                        cursor.save();
                        match indent::Parse_Opt_Or_Skip_At(cursor) {
                            Indents::Increase(token) => {
                                result.add_child(token);
                            }
                            Indents::Current(token) => {
                                if !indent_increased {
                                    cursor.restore();
                                    return result.end_at(cursor.prev_non_ws_pos()).end();
                                } else {
                                    result.add_child(token);
                                }
                            }
                            Indents::Decrease(_) => {
                                cursor.restore();
                                return result.end_at(cursor.prev_non_ws_pos()).end();
                            }
                            _ => {}
                        }
                        cursor.pop();

                        // // post-operator indent attributes
                        // if let Some(attrs) = attribute::trailing::Parser::Try_Parse_At(cursor) {
                        //     result.add_child(attrs);

                        //     if !cursor.indent().curr < base_indent {
                        //         return End::Indent_Mismatch(
                        //             "value",
                        //             base_indent,
                        //             cursor.indent().curr
                        //         );
                        //     }

                        //     // post-operator attributes indent
                        //     cursor.save();
                        //     match indent::Parse_Opt_Or_Skip_At(cursor) {
                        //         Indents::Increase(token) => {
                        //             result.add_child(token);
                        //         }
                        //         Indents::Current(token) => {
                        //             if !indent_increased {
                        //                 cursor.restore();
                        //                 return result.end_at(cursor.prev_non_ws_pos()).end();
                        //             } else {
                        //                 result.add_child(token);
                        //             }
                        //         }
                        //         Indents::Decrease(_) => {
                        //             cursor.restore();
                        //             return result.end_at(cursor.prev_non_ws_pos()).end();
                        //         }
                        //         _ => {}
                        //     }
                        //     cursor.pop();
                        // }

                        // value
                        let value = if cursor.curr_indent() > base_indent {
                            tree::Parser::Parse_Opt_At(cursor)
                        } else {
                            branch::Parser::Parse_Opt_At(cursor)
                        };

                        match value {
                            Parsed::Pass(value) => {
                                result.set_prop("value", value);

                                // post-value attributes
                                if cursor.curr_is_ws() {
                                    if let Some(trailing_attributes) = attribute::trailing::Parser::Try_Parse_At(cursor) {
                                        result.add_child(trailing_attributes);
                                    }
                                }

                                return result.end_at(cursor.prev_non_ws_pos()).end();
                            }
                            Parsed::Fail(error) => return End::Error_In_Prop_Of(result, "value", error),
                        }
                    }
                    Parsed::Fail(error) => return End::Error_In_Prop_Of(result, "operator", error),
                }
            }
            Parsed::Fail(error) => return End::Error_In_Prop_Of(result, "key", error),
        }
    },
    tests:
        pattern!(["One Line"]
            : "{name}{assigner}{expression}"
            => Token::New()
                .name(&KEY)
                .partial()
                .prop("key", Mock::Sub::<name::Parser>())
                .prop("operator", Mock::Sub::<assigner::field::Parser>())
                .prop("value", Mock::Sub::<expression::Parser>()))
        pattern!(["One Line" & "Attribute Before Key"]
            : "{attribute} {name}{assigner}{expression}"
            => Token::New()
                .name(&KEY)
                .child(Mock::Sub::<attribute::Parser>())
                .prop("key", Mock::Sub::<name::Parser>())
                .prop("operator", Mock::Sub::<assigner::field::Parser>())
                .prop("value", Mock::Sub::<expression::Parser>()))
        pattern!(["One Line" & "Attribute After Key"]
            : "{name} {attribute} {assigner}{expression}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Parser>())
                .child(Mock::Sub::<attribute::Parser>())
                .prop("operator", Mock::Sub::<assigner::field::Parser>())
                .prop("value", Mock::Sub::<expression::Parser>()))
        pattern!(["One Line" & "Attribute Before Value"]
            : "{name}{assigner}{attribute} {expression}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Parser>())
                .prop("operator", Mock::Sub::<assigner::field::Parser>())
                .child(Mock::Sub::<attribute::Parser>())
                .prop("value", Mock::Sub::<expression::Parser>()))
        pattern!(["Two Lines" & "Attribute After Key"]
            : "{name} {attribute}\n\t{assigner}{expression}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Parser>())
                .child(Mock::Sub::<attribute::Parser>())
                .prop("operator", Mock::Sub::<assigner::field::Parser>())
                .prop("value", Mock::Sub::<expression::Parser>()))
        pattern!(["Two Lines" & "Indent Increased After Assigner"]
            : "{name}{assigner}{increase_indent}{expression}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Parser>())
                .prop("operator", Mock::Sub::<assigner::field::Parser>())
                .child(Mock::Sub::<indent::increase::Parser>())
                .prop("value", Mock::Sub::<expression::Parser>()))
        pattern!(["Three Lines" & "Indent Increased Before Assigner"]
            : "{name}{increase_indent}{assigner}{current_indent}{expression}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Parser>())
                .child(Mock::Sub::<indent::increase::Parser>())
                .prop("operator", Mock::Sub::<assigner::field::Parser>())
                .child(Mock::Sub::<indent::current::Parser>())
                .prop("value", Mock::Sub::<expression::Parser>()))
        pattern!(["Three Lines" & "Indent Increased Before Assigner" & "Indent Increased After Assigner"]
            : "{name}{increase_indent}{assigner}{increase_indent}{expression}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Parser>())
                .child(Mock::Sub::<indent::increase::Parser>())
                .prop("operator", Mock::Sub::<assigner::field::Parser>())
                .child(Mock::Sub::<indent::increase::Parser>())
                .prop("value", Mock::Sub::<expression::Parser>()))

}

// fn check_for_attrs(result: &mut TokenBuilder, cursor: &mut Cursor) -> Option<bool> {
//     use crate::{
//         parser::{cursor::Cursor, results::token::Token, tokens::indent::Indents, Parser},
//         parser::{
//             results::{builder::Builder, node::Node, parsed::Parsed},
//             tokens::{attribute, expression, indent, token},
//         },
//     };
//     let base_indent = cursor.indent().curr;
//     let mut found = false;

//     while let Parsed::Pass(attribute) = attribute::Parser::Parse_Opt_At(cursor) {
//         found = true;
//         result.add_child(attribute);

//         let indent = indent::Parse_Opt_At(cursor);
//         match indent {
//             Indents::Increase(token) => {
//                 result.add_child(token);
//             }
//             Indents::Current(token) => {
//                 result.add_child(token);
//             }
//             Indents::Decrease(_) => {
//                 if cursor.indent().curr < base_indent {
//                     return None;
//                 }
//             }
//             _ => {
//                 if !(cursor.curr_is_ws() || cursor.curr_is(',')) {
//                     break;
//                 } else {
//                     cursor.skip();
//                     cursor.skip_ws();
//                 }
//             }
//         }
//     }

//     return Some(found);
// }
