use crate::parser::tokens::{statement::expression::literal::identifier::key::name, token};

token! {
  tag => |cursor: &mut Cursor| {
    match cursor.curr() {
      '#' => {
        cursor.read();
        End::Child::<tokens::statement::expression::Parser>(&KEY, cursor)
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
          .build(1, 3)
        ).build(0, 3)))
}

// tests! {
//   ["Named"]
//     : "#tag"
//     => Parsed::Pass(Token::New()
//       .name(&KEY)
//       .child(Token::New()
//         .name(name::KEY)
//         .build(1, 3)
//       ).build(0, 3))
// }
