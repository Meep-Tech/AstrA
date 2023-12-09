use crate::{
    lexer::parsers::indent::Indents,
    lexer::parsers::{indent, mutable_field_assigner, name, parser, statement::expression},
};

parser! {
    #testable,
    named_entry => |cursor: &mut Cursor | {
        let mut result = Token::New();

        let key = name::Parser::Parse_At(cursor);
        let mut indent_increased = false;
        match key {
            Parsed::Pass(key) => {
                result = result.prop("key", key);
                match indent::Parse_Opt_Or_Skip_At(cursor) {
                    Indents::Increase(token) => {
                        result = result.child(token);
                        indent_increased = true;
                    }
                    Indents::Decrease(_) => {
                        return result.end();
                    }
                    Indents::Current(_) => {
                        return result.end();
                    }
                    _ => {}
                }

                let operator = mutable_field_assigner::Parser::Parse_At(cursor);
                match operator {
                    Parsed::Pass(operator) => {
                        result = result.prop("operator", operator);
                        match indent::Parse_Opt_Or_Skip_At(cursor) {
                            Indents::Increase(token) => {
                                result = result.child(token);
                            }
                            Indents::Current(token) => {
                                if !indent_increased {
                                    return result.end();
                                } else {
                                    result = result.child(token);
                                }
                            }
                            Indents::Decrease(_) => {
                                return result.end();
                            }
                            _ => {}
                        }

                        let value = expression::Parser::Parse_At(cursor);

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
