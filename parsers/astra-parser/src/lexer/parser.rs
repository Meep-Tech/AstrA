use super::{
    cursor::Cursor,
    results::{builder::Builder, end::End, parsed::Parsed, token::Token},
};

pub trait Parser {
    fn get_name(&self) -> &'static str;
    fn has_tag(&self, tag: String) -> bool {
        return self.get_name() == tag;
    }

    fn rule(&self, start: &mut Cursor) -> Option<End>;

    fn parse(&self, input: &str) -> Option<Parsed> {
        let mut cursor = Cursor::new(input);
        return self.parse_at(&mut cursor);
    }

    fn parse_at(&self, cursor: &mut Cursor) -> Option<Parsed> {
        let start = cursor.save();
        println!("TRY: {} @ {}", self.get_name(), cursor.pos);
        match self.rule(cursor) {
            Some(result) => match result {
                End::Match(token) => {
                    println!("MATCH: {} @ {}", self.get_name(), cursor.pos);
                    Some(Parsed::Token(
                        token
                            .assure_name(self.get_name())
                            .build(start.pos, cursor.pos),
                    ))
                }
                End::Fail(error) => {
                    println!("FAIL: {} @ {}", self.get_name(), cursor.pos);
                    let err = error.tag(self.get_name()).build(start.pos, cursor.pos);
                    let result = Some(Parsed::Error(err));
                    cursor.restore(start);

                    result
                }
            },
            None => {
                println!("NONE: {} @ {}", self.get_name(), cursor.pos);
                cursor.restore(start);
                None
            }
        }
    }

    fn try_parse_at(&self, cursor: &mut Cursor) -> Option<Token> {
        let start = cursor.save();
        println!("TRY: {} @ {}", self.get_name(), cursor.pos);
        match self.rule(cursor) {
            Some(result) => match result {
                End::Match(token) => {
                    println!("MATCH: {} @ {}", self.get_name(), cursor.pos);
                    Some(
                        token
                            .assure_name(self.get_name())
                            .build(start.pos, cursor.pos),
                    )
                }
                End::Fail(_) => {
                    println!("FAIL: {} @ {}", self.get_name(), cursor.pos);
                    cursor.restore(start);
                    None
                }
            },
            None => {
                println!("NONE: {} @ {}", self.get_name(), cursor.pos);
                cursor.restore(start);
                None
            }
        }
    }
}
