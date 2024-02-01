// #region Styles

#[derive(PartialEq, Eq)]
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
    pub fn code(self) -> u8 {
        return _get_escape_code_for_color(self);
    }

    pub fn escape(self) -> String {
        return _escape_color(self);
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
        return _escape_effect(self);
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

// #endregion

// #region Styleable String Implementations

pub trait Styleable {
    fn color(&self, color: Color) -> String;
    fn color_rgb(&self, r: u8, g: u8, b: u8) -> String;
    fn bg(&self, color: Color) -> String;
    fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String;
    fn indent(&self, indent: usize) -> String;
    fn effect(&self, effect: Effect) -> String;
    fn style(&self, color: Color, bg: Color, effect: Effect) -> String {
        return self.color(color).bg(bg).effect(effect);
    }
}

impl Styleable for String {
    fn color(&self, color: Color) -> String {
        _color(color, self)
    }
    fn color_rgb(&self, r: u8, g: u8, b: u8) -> String {
        _color_rgb(r, g, b, self)
    }
    fn bg(&self, color: Color) -> String {
        _bg(self, color)
    }
    fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String {
        _bg_rgb(self, r, g, b)
    }
    fn indent(&self, indent: usize) -> String {
        _indent(self, indent)
    }
    fn effect(&self, effect: Effect) -> String {
        _effect(effect, self)
    }
}

impl Styleable for &str {
    fn color(&self, color: Color) -> String {
        _color(color, self)
    }
    fn color_rgb(&self, r: u8, g: u8, b: u8) -> String {
        _color_rgb(r, g, b, self)
    }
    fn bg(&self, color: Color) -> String {
        _bg(self, color)
    }
    fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String {
        _bg_rgb(self, r, g, b)
    }
    fn indent(&self, indent: usize) -> String {
        _indent(self, indent)
    }
    fn effect(&self, effect: Effect) -> String {
        _effect(effect, self)
    }
}

// #endregion

// #region Internal

static mut _KEYS: Vec<String> = Vec::new();

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

fn _escape_color(color: Color) -> String {
    return format!("\x1b[{}m", _get_escape_code_for_color(color));
}

fn _escape_rgb_color(r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[38;2;{};{};{}m", r, g, b);
}

fn _escape_bg(color: Color) -> String {
    return format!("\x1b[{}m", _get_escape_code_for_bg(color));
}

fn _escape_rgb_bg(r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[48;2;{};{};{}m", r, g, b);
}

fn _escape_effect(effect: Effect) -> String {
    return format!("\x1b[{}m", _get_escape_code_for_effect(effect));
}

fn _escape_reset() -> String {
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

fn _get_keys(input: &[&str], prefix: &str) -> Vec<String> {
    let mut keys = vec![prefix.to_string()];
    // add global keys
    keys.extend(unsafe { _KEYS.to_vec() });
    // add local keys
    keys.extend(
        input
            .iter()
            .map(|key| key.to_string())
            .collect::<Vec<String>>(),
    );
    return keys;
}

// #endregion

fn _color(color: Color, message: &str) -> String {
    return format!("{}{}{}", _escape_color(color), message, _escape_reset());
}

fn _color_rgb(r: u8, g: u8, b: u8, message: &str) -> String {
    return format!(
        "{}{}{}",
        _escape_rgb_color(r, g, b),
        message,
        _escape_reset()
    );
}

fn _bg(message: &str, color: Color) -> String {
    return format!("{}{}{}", _escape_bg(color), message, _escape_reset());
}

fn _bg_rgb(message: &str, r: u8, g: u8, b: u8) -> String {
    return format!("{}{}{}", _escape_rgb_bg(r, g, b), message, _escape_reset());
}

fn _effect(effect: Effect, message: &str) -> String {
    return format!("{}{}{}", _escape_effect(effect), message, _escape_reset());
}

fn _indent(text: &str, indent: usize) -> String {
    text.replace('\n', &format!("\n{}", "\t".repeat(indent)))
}
