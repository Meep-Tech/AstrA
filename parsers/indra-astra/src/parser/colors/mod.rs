use crate::{
    parser::{
        results::{node::Node, r#match::Match},
        tokens::statement::{assignment::entry, expression::literal::structure},
    },
    utils::log::{Color, Effect},
};

pub fn ascii(src: &str, lexed: &Match) -> String {
    let mut result = String::new();
    let start = lexed.start;
    let end = lexed.end;
    let cat = Category::For(&lexed);
    let color = cat.color();
    let bg = cat.bg();
    let effect = cat.effect();

    let opening_tags = format!("{}{}{}", color.escape(), bg.escape(), effect.escape());

    for child in &lexed.children {
        result.push_str(&ascii(src, child));
    }

    let end_tags = format!(
        "{}{}{}",
        Color::Reset.escape(),
        Color::Reset.escape(),
        Effect::Reset.escape()
    );

    result.push_str(&format!("{}{}{}", &src[start..end], end_tags, opening_tags));
    result
}

pub enum Category {
    FieldDeclaration,
    ProcDeclaration,
    FieldIdentifier,
    ProcIdentifier,
    StringLiteral,
    NumberLiteral,
    LiteralEscape,
    NakedLiteral,
    LogicalOperator,
    AssignmentOperator,
    StructureLiteral,
    EntryModifier,
    TagAttribute,
    AliasAttribute,
}

impl Category {
    #[allow(non_snake_case)]
    pub fn For(token: &Match) -> Category {
        if token.tag(entry::KEY) {
            return Category::FieldDeclaration;
        } else if token.tag(structure::KEY) {
            return Category::StringLiteral;
        } else {
            panic!("Unknown category for token: {:?}", token);
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Category::FieldDeclaration => Color::BrightCyan,
            Category::ProcDeclaration => Color::BrightYellow,
            Category::FieldIdentifier => Color::Cyan,
            Category::ProcIdentifier => Color::Yellow,
            Category::StringLiteral => Color::Green,
            Category::LiteralEscape => Color::BrightYellow,
            Category::NumberLiteral => Color::BrightBlue,
            Category::NakedLiteral => Color::BrightWhite,
            Category::LogicalOperator => Color::Red,
            Category::AssignmentOperator => Color::BrightRed,
            Category::StructureLiteral => Color::BrightGreen,
            Category::EntryModifier => Color::Magenta,
            Category::TagAttribute => Color::BrightMagenta,
            Category::AliasAttribute => Color::White,
        }
    }

    pub fn bg(&self) -> Color {
        match self {
            _ => Color::Reset,
        }
    }

    pub fn effect(&self) -> Effect {
        match self {
            Category::FieldDeclaration => Effect::Underline,
            Category::ProcDeclaration => Effect::Underline,
            _ => Effect::Reset,
        }
    }
}
