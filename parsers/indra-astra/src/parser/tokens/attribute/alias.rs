use crate::parser::tokens::{expression::identifier::key::name, token};

token! {
  alias => |cursor: &mut Cursor| {
    match cursor.curr() {
      '|' => {
        cursor.read();
        match cursor.curr() {
          '>' => End::ToDo("read as input-only-alias"),
          '#' => End::ToDo("read as trait-override-alias"),
          _ => End::Child::<tokens::expression::attribute_expression::Parser>(&KEY, cursor)
        }
      },
      _ => End::Missing(&KEY, "|", &cursor.curr_str())
    }
  },
  tests:
    unit!(["Named"]
      : "|alias"
      => Parsed::Pass(Token::New()
        .name(&KEY)
        .child(Token::New()
          .name(name::KEY)
          .partial()
          .build_from(1, 5)
        ).build_from(0, 5)))
}
