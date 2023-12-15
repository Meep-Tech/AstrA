use crate::parser::{
    cursor::Cursor,
    results::{r#match::Match, token_builder::TokenBuilder},
    tokens::{
        attribute,
        statement::expression::{self, invocation::identifier::key::name},
        symbol::operator::assigner::mutable_field_assigner,
        token,
        whitespace::indent::{self, Indents},
    },
};

token! {
    named_entry => |cursor: &mut Cursor | {
        let mut result = Match::New();

        // pre-key attributes
        let mut base_indent = cursor.indents.curr;
        if let Some(_) = check_for_attrs(&mut result, cursor) {
            if !cursor.indents.curr == base_indent {
                return End::Mismatch(
                    "size-for-key-indent",
                    &format!("{}", base_indent),
                    &format!("{}", cursor.indents.curr),
                );
            }
        } else {
            return result.end();
        }

        // key
        let key = name::Token::Parse_At(cursor);
        let mut indent_increased = false;
        match key {
            Parsed::Pass(key) => {
                result.set_prop("key", key);

                // post-key indent
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

                // post-key attributes
                if cursor.prev_is_ws() {
                    if let Some(attrs) = check_for_attrs(&mut result, cursor) {
                        if attrs {
                            // post-key attributes indent
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
                        }
                    } else {
                        return result.end();
                    }
                }


                // operator
                let operator = mutable_field_assigner::Token::Parse_At(cursor);
                match operator {
                    Parsed::Pass(operator) => {
                        result.set_prop("operator", operator);

                        // post-operator indent
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

                        // post-operator attributes
                        base_indent = cursor.indents.curr;
                        if let Some(attrs) = check_for_attrs(&mut result, cursor) {
                            if !cursor.indents.curr < base_indent {
                                return End::Mismatch(
                                    "size-for-value-indent",
                                    &format!("{}", base_indent),
                                    &format!("{}", cursor.indents.curr),
                                );
                            }

                            if attrs {
                                // post-operator attributes indent
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
                            }
                        } else {
                            return result.end();
                        }

                        // value
                        let value = expression::entry_expression::Token::Parse_At(cursor);
                        match value {
                            Parsed::Pass(value) => {
                                result.set_prop("value", value);

                                // post-value attributes
                                if cursor.curr_is_ws() {
                                    cursor.skip_ws();
                                    check_for_attrs(&mut result, cursor);
                                }

                                return result.end();
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
            : "{%name%}{%assigner%}{%value%}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Token>())
                .prop("operator", Mock::Sub::<mutable_field_assigner::Token>())
                .prop("value", Mock::Sub::<expression::entry_expression::Token>()))
        pattern!(["One Line" & "Attribute Before Key"]
            : "{%attribute%} {%name%}{%assigner%}{%value%}"
            => Token::New()
                .name(&KEY)
                .child(Mock::Sub::<attribute::Token>())
                .prop("key", Mock::Sub::<name::Token>())
                .prop("operator", Mock::Sub::<mutable_field_assigner::Token>())
                .prop("value", Mock::Sub::<expression::entry_expression::Token>()))
        pattern!(["One Line" & "Attribute After Key"]
            : "{%name%} {%attribute%} {%assigner%}{%value%}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Token>())
                .child(Mock::Sub::<attribute::Token>())
                .prop("operator", Mock::Sub::<mutable_field_assigner::Token>())
                .prop("value", Mock::Sub::<expression::entry_expression::Token>()))
        pattern!(["One Line" & "Attribute Before Value"]
            : "{%name%}{%assigner%}{%attribute%} {%value%}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Token>())
                .prop("operator", Mock::Sub::<mutable_field_assigner::Token>())
                .child(Mock::Sub::<attribute::Token>())
                .prop("value", Mock::Sub::<expression::entry_expression::Token>()))
        pattern!(["Two Lines" & "Indent Increased After Assigner"]
            : "{%name%}{%assigner%}{%increase_indent%}{%value%}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Token>())
                .prop("operator", Mock::Sub::<mutable_field_assigner::Token>())
                .child(Mock::Sub::<indent::increase::Token>())
                .prop("value", Mock::Sub::<expression::entry_expression::Token>()))
        pattern!(["Three Lines" & "Indent Increased Before Assigner"]
            : "{%name%}{%increase_indent%}{%assigner%}{%current_indent%}{%value%}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Token>())
                .child(Mock::Sub::<indent::increase::Token>())
                .prop("operator", Mock::Sub::<mutable_field_assigner::Token>())
                .child(Mock::Sub::<indent::current::Token>())
                .prop("value", Mock::Sub::<expression::entry_expression::Token>()))
        pattern!(["Three Lines" & "Indent Increased Before Assigner" & "Indent Increased After Assigner"]
            : "{%name%}{%increase_indent%}{%assigner%}{%increase_indent%}{%value%}"
            => Token::New()
                .name(&KEY)
                .prop("key", Mock::Sub::<name::Token>())
                .child(Mock::Sub::<indent::increase::Token>())
                .prop("operator", Mock::Sub::<mutable_field_assigner::Token>())
                .child(Mock::Sub::<indent::increase::Token>())
                .prop("value", Mock::Sub::<expression::entry_expression::Token>()))

}

fn check_for_attrs(result: &mut TokenBuilder, cursor: &mut Cursor) -> Option<bool> {
    use crate::{
        parser::{cursor::Cursor, results::r#match::Match, tokens::indent::Indents, Type},
        parser::{
            results::{builder::Builder, node::Node, parsed::Parsed},
            tokens::{
                attribute, indent, mutable_field_assigner, name, statement::expression, token,
            },
        },
    };
    let base_indent = cursor.indents.curr;
    let mut found = false;

    while let Parsed::Pass(attribute) = attribute::Token::Parse_Opt_At(cursor) {
        found = true;
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
                    return None;
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

    return Some(found);
}
