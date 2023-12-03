use crate::utils::log::{Color, Effect};

pub enum Category {
    FieldDeclaration,
    ProdDeclaration,
    FieldIdentifier,
    ProcIdentifier,
    StringLiteral,
    NumberLiteral,
    LiteralEscape,
    NakedLiteral,
    LogicalOperator,
    AssignmentOperator,
    StructuralDelimiter,
    EntryModifier,
    TagAttribute,
    AliasAttribute,
}

impl Category {
    pub fn color(&self) -> Color {
        match self {
            Category::FieldDeclaration => Color::BrightCyan,
            Category::ProdDeclaration => Color::BrightYellow,
            Category::FieldIdentifier => Color::Cyan,
            Category::ProcIdentifier => Color::Yellow,
            Category::StringLiteral => Color::Green,
            Category::LiteralEscape => Color::BrightYellow,
            Category::NumberLiteral => Color::BrightBlue,
            Category::NakedLiteral => Color::BrightWhite,
            Category::LogicalOperator => Color::Red,
            Category::AssignmentOperator => Color::BrightRed,
            Category::StructuralDelimiter => Color::BrightGreen,
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
            Category::ProdDeclaration => Effect::Underline,
            _ => Effect::Reset,
        }
    }
}
