use crate::lexer::{
    cursor::Cursor,
    parsers::{
        attribute, parser,
        statement::expression::{self, invocation::identifier::key::name},
        symbol::operator::assigner::mutable_field_assigner,
        whitespace::indent::{self, Indents},
    },
    results::{token::Token, token_builder::TokenBuilder},
};

parser! {
    #testable,
    named_entry => |cursor: &mut Cursor | {
        let mut result = Token::New();
        let base_indent = cursor.indents.curr;

        check_for_attrs(&mut result, cursor);

        if !cursor.indents.curr == base_indent {
            return End::Mismatch(
                "key-indent",
                &format!("{} indents", base_indent),
                &format!("{} indents", cursor.indents.curr),
            );
        }

        let key = name::Parser::Parse_At(cursor);
        let mut indent_increased = false;
        match key {
            Parsed::Pass(key) => {
                result.set_prop("key", key);
                match indent::Parse_Opt_Or_Skip_At(cursor) {
                    Indents::Increase(token) => {
                        result.add_child(token);
                        indent_increased = true;
                    }
                    Indents::Decrease(_) => {
                        return result.end();
                    }
                    Indents::Current(_) => {
                        return result.end();
                    }
                    _ => {
                        cursor.skip_ws();
                    }
                }

                if cursor.prev_is_ws() {
                    while let Parsed::Pass(attribute) = attribute::Parser::Parse_Opt_At(cursor) {
                        result.add_child(attribute);
                    }
                }

                let operator = mutable_field_assigner::Parser::Parse_At(cursor);
                match operator {
                    Parsed::Pass(operator) => {
                        result.set_prop("operator", operator);
                        match indent::Parse_Opt_Or_Skip_At(cursor) {
                            Indents::Increase(token) => {
                                result.add_child(token);
                            }
                            Indents::Current(token) => {
                                if !indent_increased {
                                    return result.end();
                                } else {
                                    result.add_child(token);
                                }
                            }
                            Indents::Decrease(_) => {
                                return result.end();
                            }
                            _ => {}
                        }

                        let value = expression::entry_expression::Parser::Parse_At(cursor);

                        match value {
                            Parsed::Pass(value) => {
                                return result.prop("value", value).end();
                            }
                            Parsed::Fail(error) => {
                                return End::Error_In_Prop_Of(result, "value", error)
                            }
                        }
                    }
                    Parsed::Fail(error) => return End::Error_In_Prop_Of(result, "operator", error),
                }
            }
            Parsed::Fail(error) => return End::Error_In_Prop_Of(result, "key", error),
        }
    }
}

fn check_for_attrs(result: &mut TokenBuilder, cursor: &mut Cursor) -> bool {
    use crate::{
        init,
        lexer::{cursor::Cursor, parser::Parser, parsers::indent::Indents, results::token::Token},
        lexer::{
            parsers::{
                attribute, indent, mutable_field_assigner, name, parser, statement::expression,
            },
            results::{builder::Builder, node::Node, parsed::Parsed},
        },
    };
    let base_indent = cursor.indents.curr;

    while let Parsed::Pass(attribute) = attribute::Parser::Parse_Opt_At(cursor) {
        result.add_child(attribute);

        let indent = indent::Parse_Opt_At(cursor);
        match indent {
            Indents::Increase(token) => {
                result.add_child(token);
            }
            Indents::Current(token) => {
                result.add_child(token);
            }
            Indents::Decrease(_) => {
                if cursor.indents.curr < base_indent {
                    return false;
                }
            }
            _ => {
                if !(cursor.curr_is_ws() || cursor.curr_is(',')) {
                    break;
                } else {
                    cursor.skip();
                    cursor.skip_ws();
                }
            }
        }
    }

    return true;
}
