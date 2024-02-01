use crate::parser::tokens::{expression::identifier::key::name, token};

token! {
  tag => |cursor: &mut Cursor| {
    match cursor.curr() {
      '#' => {
        cursor.read();
        End::Child::<tokens::expression::attribute_expression::Parser>(&KEY, cursor)
      },
      '>' => match cursor.next() {
        '#' => End::TODO(),
        '>' => match cursor.next() {
          '#' => End::TODO(),
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
          .build(1, 3)
        ).build(0, 3)))
}
