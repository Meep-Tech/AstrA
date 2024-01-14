use crate::parser::{
    term::{
        cats::{Whitespace, Word},
        Type,
    },
    Term,
};

use self::ansi::Styleable;

pub mod ansi;

pub fn terms_via_ansi(src: &str, terms: &Vec<Term>) -> String {
    let mut out = String::new();
    let mut t = 0;

    for i in 0..src.len() {
        if t >= terms.len() {
            out.push_str(&src[terms.last().unwrap().end..]);
            break;
        }

        let mut term = &terms[t];
        let mut dedent_encountered = false;
        if term.start == i {
            out.push_str(&term_via_ansi(&src, term));
            if term.ttype == Type::Whitespace(Whitespace::Dedent) {
                dedent_encountered = true;
            }
        } else if i < term.start {
            out.push(src.chars().nth(i).unwrap());
        }

        if i == term.end && t < terms.len() - 1 {
            t += 1;
            term = &terms[t];
            while i == term.end && t < terms.len() - 1 {
                out.push_str(&term_via_ansi(&src, term));
                if term.ttype == Type::Whitespace(Whitespace::Dedent) {
                    dedent_encountered = true;
                }
                t += 1;
                term = &terms[t];
            }
        }

        if dedent_encountered {
            out.push_str("\n");
        }
    }

    out
}

pub fn term_via_ansi(src: &str, term: &Term) -> String {
    let rgb = for_term(term);
    let text = term.text_from(src);

    if let Some((r, g, b)) = rgb {
        text.color_rgb(r, g, b)
    } else if term.is_ws() {
        match &term.ttype {
            Type::Whitespace(ws) => match ws {
                Whitespace::Indent => {
                    if text == "\t" { "⟼  " } else { "↦ " }.color(ansi::Color::BrightGreen)
                }
                Whitespace::Dedent => " ↤".color(ansi::Color::BrightGreen),
            },
            _ => panic!("Expected whitespace, found {:?}", term.ttype),
        }
    } else {
        text
    }
}

pub fn for_term(term: &Term) -> Option<(u8, u8, u8)> {
    match &term.ttype {
        Type::Word(wtype) => match &wtype {
            Word::Whole => Some((128, 212, 255)),
            Word::Delimited => Some((102, 179, 255)),
        },
        Type::Number => Some((0, 128, 255)),
        Type::Operator(_) => Some((230, 0, 230)),
        Type::Delimiter(_) => Some((255, 255, 0)),
        Type::Ambiguous(_) => Some((255, 149, 128)),
        Type::Reserved => Some((255, 0, 0)),
        _ => None,
    }
}

// pub fn highlight(src: &str, tokens: &Vec<Token>) -> String {
//     let mut out = String::new();
//     let mut last = 0;

//     for token in tokens {
//         out.push_str(&src[last..token.start]);
//         out.push_str(&token.highlight());
//         last = token.end;
//     }

//     out.push_str(&src[last..]);

//     out
// }
