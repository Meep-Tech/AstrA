use crate::parser::tokens::{expression::identifier::key::name, token};

token! {
  input => |cursor: &mut Cursor| {
    match cursor.curr() {
      '>' => {
        cursor.read();
        match cursor.curr() {
          '>' => End::Invalid("second-right-angle-bracket-in-initial-input-attribute", "`>>` is not a valid prefix for an input attribute, as it is reserved for the procedural assigner."),
          '#' => End::ToDo("read as typed-input"),
          '|' => End::ToDo("read as alt-input"),
          '(' => End::ToDo("read as input group"),
          '[' => End::ToDo("read as input block"),
          '{' => End::ToDo("read as input map"),
          '<' => End::ToDo("read as input generic"),
          _ => End::Child::<tokens::expression::attribute_expression::Parser>(&KEY, cursor)
        }
      },
      _ => End::Missing(&KEY, ">", &cursor.curr_str())
    }
  },
  tests:
    unit!(["Named"]
      : ">input"
      => Parsed::Pass(Token::New()
        .name(&KEY)
        .child(Token::New()
          .name(name::KEY)
          .partial()
          .build(1, 5)
        ).build(0, 5)))
}
