use std::{
    cell::{LazyCell, RefCell},
    collections::HashMap,
};

static mut _KEYS: Vec<String> = Vec::new();
static mut _STYLES: LazyCell<RefCell<HashMap<String, String>>> =
    LazyCell::new(|| RefCell::new(HashMap::new()));

pub fn log(keys: &[&str], message: &str) {
    if Some(unsafe { &_STYLES }).is_some() {
        unsafe {
            let message_parts = message.split_whitespace().collect::<Vec<&str>>();
            let mut message_splitters: Vec<(usize, &str)> =
                message.match_indices(|c: char| c.is_whitespace()).collect();
            let mut styled_keys: Vec<String> = Vec::new();
            for key in keys {
                if _STYLES.borrow().contains_key(*key) {
                    styled_keys.push(_STYLES.borrow().get(*key).unwrap().clone());
                } else {
                    styled_keys.push(key.to_string());
                }
            }
            let mut styled_message = String::new();
            for part in message_parts {
                if _STYLES.borrow().contains_key(part) {
                    styled_message.push_str(_STYLES.borrow().get(part).unwrap());
                } else {
                    styled_message.push_str(part);
                }

                if !message_splitters.is_empty() {
                    let mut next_splitter = message_splitters.remove(0);
                    styled_message.push_str(next_splitter.1);
                    while (!message_splitters.is_empty())
                        && ((message_splitters.first().unwrap().0 - next_splitter.0) == 1)
                    {
                        next_splitter = message_splitters.remove(0);
                        styled_message.push_str(next_splitter.1);
                    }
                };
            }

            println!("[{}]: {}", styled_keys.join("]["), styled_message);
        }
    } else {
        println!("[{}]: {}", keys.join("]["), message);
    }
}

pub fn set_bg(text: &str, bg: Color) {
    let text = text.to_string();
    unsafe {
        if _STYLES.borrow().contains_key(&text) {
            _STYLES
                .borrow_mut()
                .insert(text.clone(), format!("{}{}", escape_bg(bg), text));
        } else {
            _STYLES.borrow_mut().insert(
                text.clone(),
                format!("{}{}{}", escape_bg(bg), text, escape_reset()),
            );
        }
    }
}

pub fn set_color(text: &str, color: Color) {
    let text = text.to_string();
    unsafe {
        if _STYLES.borrow().contains_key(&text) {
            _STYLES
                .borrow_mut()
                .insert(text.clone(), format!("{}{}", escape_color(color), text));
        } else {
            _STYLES.borrow_mut().insert(
                text.clone(),
                format!("{}{}{}", escape_color(color), text, escape_reset()),
            );
        }
    }
}

pub fn set_effect(text: &str, effect: Effect) {
    let text = text.to_string();
    unsafe {
        if _STYLES.borrow().contains_key(&text) {
            _STYLES
                .borrow_mut()
                .insert(text.clone(), format!("{}{}", escape_effect(effect), text));
        } else {
            _STYLES.borrow_mut().insert(
                text.clone(),
                format!("{}{}{}", escape_effect(effect), text, escape_reset()),
            );
        }
    }
}

pub fn push_unique_key(key: &str) {
    let key = key.to_string();
    unsafe {
        if !_KEYS.contains(&key) {
            _KEYS.push(key);
        }
    }
}

pub fn info(keys: &[&str], message: &str) {
    let info_separator = "-".color(Color::BrightBlue);
    log(
        get_keys(keys, &info_separator)
            .iter()
            .map(|key| key.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
        message,
    );
}

pub fn warn(keys: &[&str], message: &str) {
    let warn_separator = "*".color(Color::BrightYellow);
    log(
        &get_keys(keys, &warn_separator)
            .iter()
            .map(|key| key.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
        message,
    );
}

pub fn error(keys: &[&str], message: &str) {
    let error_prefix = "!".color(Color::BrightRed);
    log(
        &get_keys(keys, &error_prefix)
            .iter()
            .map(|key| key.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
        message,
    );
}

pub fn push_key(key: &str) {
    unsafe {
        _KEYS.push(key.to_string());
    }
}

pub fn push_key_div(separator: &str, color: Color) {
    unsafe {
        _KEYS.push(separator.color(color));
    }
}

pub fn pop_key() {
    unsafe {
        _KEYS.pop();
    }
}

pub fn pop_unique_key(key: &str) {
    unsafe {
        if _KEYS.last().unwrap_or(&"".to_string()) == key {
            _KEYS.pop();
        }
    }
}

pub fn color(color: Color, message: &str) -> String {
    return format!("{}{}{}", escape_color(color), message, escape_reset());
}

pub fn indent(text: &str, indent: usize) -> String {
    text.replace('\n', &format!("\n{}", "\t".repeat(indent)))
}

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
}

impl Color {
    pub fn code(self) -> u8 {
        return get_escape_code_for_color(self);
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

pub trait Colorable {
    fn color(&self, color: Color) -> String;
}

pub trait Indentable {
    fn indent(&self, indent: usize) -> String;
}

impl Colorable for String {
    fn color(&self, color: Color) -> String {
        return super::log::color(color, self);
    }
}

impl Colorable for &str {
    fn color(&self, color: Color) -> String {
        return super::log::color(color, self);
    }
}

impl Indentable for String {
    fn indent(&self, indent: usize) -> String {
        return super::log::indent(self, indent);
    }
}

fn escape_color(color: Color) -> String {
    return format!("\x1b[{}m", get_escape_code_for_color(color));
}

fn escape_bg(color: Color) -> String {
    return format!("\x1b[{}m", get_escape_code_for_bg(color));
}

fn escape_effect(effect: Effect) -> String {
    return format!("\x1b[{}m", get_escape_code_for_effect(effect));
}

fn escape_reset() -> String {
    return format!("\x1b[{}m", get_escape_code_for_color(Color::Reset));
}

fn get_escape_code_for_effect(effect: Effect) -> u8 {
    match effect {
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

fn get_escape_code_for_color(color: Color) -> u8 {
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

fn get_escape_code_for_bg(color: Color) -> u8 {
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

fn get_keys(input: &[&str], prefix: &str) -> Vec<String> {
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
/*
   let mut all_keys = vec![prefix.to_string()];
   all_keys.extend(unsafe {
       _KEYS
           .iter()
           .map(|key| key.to_string())
           .collect::<Vec<String>>()
   });
   all_keys.extend(
       input
           .iter()
           .map(|key| key.to_string())
           .collect::<Vec<String>>(),
   );
   return all_keys.iter().map(|key| key.to_string()).collect();
*/
