use crate::lexer::parsers::{
    parser, statement::expression::invocation::identifier::key::name, tests,
};

parser! {
  #testable,
  tag => |cursor: &mut Cursor| {
    match cursor.curr() {
      '#' => {
        cursor.read();
        End::Child::<parsers::statement::expression::attribute_expression::Parser>(&KEY, cursor)
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
  }
}

tests! {
  ["Named"]
    : "#tag"
    => Parsed::Pass(Token::New()
      .name(&KEY)
      .child(Token::New()
        .name(name::KEY)
        .build(1, 3)
      ).build(0, 3))
}
