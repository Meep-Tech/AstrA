use crate::parser::tokens::{expression::identifier::key::name, token};

token! {
  tag => |cursor: &mut Cursor| {
    match cursor.curr() {
      '#' => {
        cursor.read();
        End::Child::<tokens::expression::attribute_expression::Parser>(&KEY, cursor)
      },
      '>' => match cursor.next() {
        '#' => End::ToDo("read as input-tag"),
        '>' => match cursor.next() {
          '#' => End::ToDo("read as output-tag"),
          _ => End::Missing(&KEY, "#", &cursor.next_str())
        },
        _ => End::Missing(&KEY, "#", &cursor.next_str())
      },
      _ => End::Missing(&KEY, "#", &cursor.curr_str())
    }
  },
  tests:
    unit!(["Named"]
      : "#tag"
      => Parsed::Pass(Token::New()
        .name(&KEY)
        .child(Token::New()
          .name(name::KEY)
          .partial()
          .build_from(1, 3)
        ).build_from(0, 3)))
}
