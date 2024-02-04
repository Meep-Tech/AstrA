// #region Stylization Methods

pub fn color(color: Color, message: &str) -> String {
    return format!("{}{}{}", escape_color(color), message, escape_reset());
}

pub fn bg(message: &str, color: Color) -> String {
    return format!("{}{}{}", escape_bg(color), message, escape_reset());
}

pub fn effect(effect: Effect, message: &str) -> String {
    return format!("{}{}{}", escape_effect(effect), message, escape_reset());
}

pub fn indent(text: &str, indent: usize) -> String {
    text.replace('\n', &format!("\n{}", "\t".repeat(indent)))
}

pub fn get_random_color(message: &str) -> Color {
    let hasher = unsafe { &mut _HASHER };
    message.to_string().hash::<DefaultHasher>(hasher);
    let color_hash: usize = {
        let finish = hasher.finish();
        (finish as usize % 8).try_into().unwrap()
    };

    return _get_color_by_ordered_number(color_hash);
}

// #endregion

// #region Styles

use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Effect {
    Bold,
    Dim,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    Invert,
    Hidden,
    Strikethrough,
    Font(u8),
    Framed,
    Encircled,
    Overlined,
    Reset,
}

impl Color {
    pub type Loop = ColorLoop;

    pub fn code(self) -> u8 {
        return _get_escape_code_for_color(self);
    }

    pub fn escape(self) -> String {
        return escape_color(self);
    }

    pub fn from_str(color: &str) -> Color {
        match color {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "bright-black" => Color::BrightBlack,
            "bright-red" => Color::BrightRed,
            "bright-green" => Color::BrightGreen,
            "bright-yellow" => Color::BrightYellow,
            "bright-blue" => Color::BrightBlue,
            "bright-magenta" => Color::BrightMagenta,
            "bright-cyan" => Color::BrightCyan,
            "bright-white" => Color::BrightWhite,
            "reset" => Color::Reset,
            _ => panic!("Unknown color: {}", color),
        }
    }
}

impl Effect {
    pub fn code(self) -> u8 {
        return _get_escape_code_for_effect(self);
    }

    pub fn escape(self) -> String {
        return escape_effect(self);
    }

    pub fn from_str(effect: &str) -> Effect {
        match effect {
            "bold" => Effect::Bold,
            "dim" => Effect::Dim,
            "italic" => Effect::Italic,
            "underline" => Effect::Underline,
            "slow-blink" => Effect::SlowBlink,
            "rapid-blink" => Effect::RapidBlink,
            "invert" => Effect::Invert,
            "hidden" => Effect::Hidden,
            "strikethrough" => Effect::Strikethrough,
            "font" => Effect::Font(0),
            "framed" => Effect::Framed,
            "encircled" => Effect::Encircled,
            "overlined" => Effect::Overlined,
            "reset" => Effect::Reset,
            _ => panic!("Unknown effect: {}", effect),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColorLoop {
    index: usize,
    colors: Vec<Color>,
}

impl ColorLoop {
    #[allow(non_snake_case)]
    pub fn New(colors: Vec<Color>) -> ColorLoop {
        return ColorLoop { index: 0, colors };
    }

    pub fn curr(&self) -> Color {
        return self.colors[self.index];
    }

    pub fn next(&mut self) -> Color {
        let color = self.colors[self.index];
        self.index = (self.index + 1) % self.colors.len();
        return color;
    }

    pub fn prev(&mut self) -> Color {
        if self.index == 0 {
            self.index = self.colors.len() - 1;
        } else {
            self.index -= 1;
        }
        return self.colors[self.index];
    }
}

// #endregion

// #region Styleable String Implementations

pub trait Styleable: Display {
    fn color(&self, color: Color) -> String;
    fn bg(&self, color: Color) -> String;
    fn indent(&self, indent: usize) -> String;
    fn effect(&self, effect: Effect) -> String;
    fn style(&self, color: Color, bg: Color, effect: Effect) -> String {
        return self.color(color).bg(bg).effect(effect);
    }
    fn own_color(&self) -> String {
        return self.color(get_random_color(self.to_string().as_str()));
    }
}

impl Styleable for String {
    fn color(&self, color: Color) -> String {
        super::ansi::color(color, self)
    }
    fn bg(&self, color: Color) -> String {
        super::ansi::bg(self, color)
    }
    fn indent(&self, indent: usize) -> String {
        super::ansi::indent(self, indent)
    }
    fn effect(&self, effect: Effect) -> String {
        super::ansi::effect(effect, self)
    }
}

impl Styleable for &str {
    fn color(&self, color: Color) -> String {
        super::ansi::color(color, self)
    }
    fn bg(&self, color: Color) -> String {
        super::ansi::bg(self, color)
    }
    fn indent(&self, indent: usize) -> String {
        super::ansi::indent(self, indent)
    }
    fn effect(&self, effect: Effect) -> String {
        super::ansi::effect(effect, self)
    }
}

// #endregion

// #region Internal
static mut _HASHER: std::collections::hash_map::DefaultHasher =
    std::collections::hash_map::DefaultHasher::new();

fn _get_color_by_ordered_number(index: usize) -> Color {
    match index {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Blue,
        4 => Color::Magenta,
        5 => Color::Cyan,
        6 => Color::White,
        7 => Color::Black,
        8 => Color::BrightRed,
        9 => Color::BrightGreen,
        10 => Color::BrightYellow,
        11 => Color::BrightBlue,
        12 => Color::BrightMagenta,
        13 => Color::BrightCyan,
        14 => Color::BrightWhite,
        _ => panic!("Unknown color index: {}", index),
    }
}

pub fn escape_color(color: Color) -> String {
    return format!("\x1b[{}m", _get_escape_code_for_color(color));
}

pub fn escape_bg(color: Color) -> String {
    return format!("\x1b[{}m", _get_escape_code_for_bg(color));
}

pub fn escape_effect(effect: Effect) -> String {
    return format!("\x1b[{}m", _get_escape_code_for_effect(effect));
}

pub fn escape_reset() -> String {
    return format!("\x1b[{}m", _get_escape_code_for_color(Color::Reset));
}

fn _get_escape_code_for_effect(effect: Effect) -> u8 {
    match effect {
        Effect::Reset => 0,
        Effect::Bold => 1,
        Effect::Dim => 2,
        Effect::Italic => 3,
        Effect::Underline => 4,
        Effect::SlowBlink => 5,
        Effect::RapidBlink => 6,
        Effect::Invert => 7,
        Effect::Hidden => 8,
        Effect::Strikethrough => 9,
        Effect::Font(n) => 10 + n,
        Effect::Framed => 51,
        Effect::Encircled => 52,
        Effect::Overlined => 53,
    }
}

fn _get_escape_code_for_color(color: Color) -> u8 {
    match color {
        Color::Black => 30,
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Magenta => 35,
        Color::Cyan => 36,
        Color::White => 37,
        Color::BrightBlack => 90,
        Color::BrightRed => 91,
        Color::BrightGreen => 92,
        Color::BrightYellow => 93,
        Color::BrightBlue => 94,
        Color::BrightMagenta => 95,
        Color::BrightCyan => 96,
        Color::BrightWhite => 97,
        Color::Reset => 0,
    }
}

fn _get_escape_code_for_bg(color: Color) -> u8 {
    match color {
        Color::Black => 40,
        Color::Red => 41,
        Color::Green => 42,
        Color::Yellow => 43,
        Color::Blue => 44,
        Color::Magenta => 45,
        Color::Cyan => 46,
        Color::White => 47,
        Color::BrightBlack => 100,
        Color::BrightRed => 101,
        Color::BrightGreen => 102,
        Color::BrightYellow => 103,
        Color::BrightBlue => 104,
        Color::BrightMagenta => 105,
        Color::BrightCyan => 106,
        Color::BrightWhite => 107,
        Color::Reset => 0,
    }
}
// #endregion
