use std::{
    cell::{LazyCell, RefCell},
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

// #region Loggers

// #region Macros

macro_rules! info {
    (v: $($rest:tt)*) => {
        #[cfg(feature = "v")]
        log::log_info($($rest)*);
    };
    (vv: $($rest:tt)*) => {
        #[cfg(feature = "vv")]
        log::log_info($($rest)*);
    };
    (vvv: $($rest:tt)*) => {
        #[cfg(feature = "vvv")]
        log::log_info($($rest)*);
    };
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::log_info($($rest)*);
    };
}
pub(crate) use info;

macro_rules! plain {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::log_info_plain($($rest)*);
    }
}
pub(crate) use plain;

macro_rules! warning {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::log_warn($($rest)*);
    }
}
pub(crate) use warning;

macro_rules! error {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::log_error($($rest)*);
    }
}
pub(crate) use error;

macro_rules! ln {
    () => {
        #[cfg(feature = "log")]
        log::log_ln();
    };
}
pub(crate) use ln;

// #endregion

pub fn log(keys: &[&str], message: &str) {
    if Some(unsafe { &_STYLES }).is_some() {
        let styled_keys = style_keys(keys);
        let styled_message = style_text(message);

        println!("[{}]: {}", styled_keys.join("]["), styled_message);
    } else {
        println!("[{}]: {}", keys.join("]["), message);
    }
}

pub fn log_plain(keys: &[&str], message: &str) {
    println!("[{}]: {}", keys.join("]["), message);
}

pub fn log_plain_message(keys: &[&str], message: &str) {
    if Some(unsafe { &_STYLES }).is_some() {
        let styled_keys = style_keys(keys);

        println!("[{}]: {}", styled_keys.join("]["), message);
    } else {
        println!("[{}]: {}", keys.join("]["), message);
    }
}

pub fn log_info_plain(keys: &[&str], message: &str) {
    let info_separator = "-".color(Color::BrightBlue);
    log_plain_message(
        _get_keys(keys, &info_separator)
            .iter()
            .map(|key| key.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
        message,
    );
}

pub fn log_info(keys: &[&str], message: &str) {
    let info_separator = "-".color(Color::BrightBlue);
    log(
        _get_keys(keys, &info_separator)
            .iter()
            .map(|key| key.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
        message,
    );
}

pub fn log_warn(keys: &[&str], message: &str) {
    let warn_separator = "*".color(Color::BrightYellow);
    log(
        &_get_keys(keys, &warn_separator)
            .iter()
            .map(|key| key.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
        message,
    );
}

pub fn log_error(keys: &[&str], message: &str) {
    let error_prefix = "!".color(Color::BrightRed);
    log(
        &_get_keys(keys, &error_prefix)
            .iter()
            .map(|key| key.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
        message,
    );
}

pub fn log_ln() {
    println!();
}

// #endregion

// #region Style Setters

// #region Macros

macro_rules! bg {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::add_bg($($rest)*);
    }
}
pub(crate) use bg;

macro_rules! color {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::add_color($($rest)*);
    }
}
pub(crate) use color;

#[allow(unused_macros)]
macro_rules! effect {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::add_effect($($rest)*);
    }
}
#[allow(unused_imports)]
pub(crate) use effect;

#[allow(unused_macros)]
macro_rules! style {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::add_style($($rest)*);
    }
}
#[allow(unused_imports)]
pub(crate) use style;

macro_rules! random_style {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::set_random_style($($rest)*);
    }
}
pub(crate) use random_style;

// #endregion

pub fn add_bg(text: &str, bg: Color) {
    add_style(text, &_escape_bg(bg));
}

pub fn add_color(text: &str, color: Color) {
    add_style(text, &_escape_color(color));
}

pub fn add_effect(text: &str, effect: Effect) {
    add_style(text, &_escape_effect(effect));
}

pub fn add_style(text: &str, style: &str) {
    let text = text.to_string();
    unsafe {
        if _STYLES.borrow().contains_key(&text) {
            _STYLES.borrow_mut().insert(
                text.clone(),
                format!("{}{}{}", style, text, _escape_reset()),
            );
            _STYLES.borrow_mut().insert(
                format!("\"{}\"", text),
                format!("\"{}{}{}\"", style, text, _escape_reset()),
            );
        } else {
            _STYLES.borrow_mut().insert(
                text.clone(),
                format!("{}{}{}", style, text, _escape_reset()),
            );
            _STYLES.borrow_mut().insert(
                format!("\"{}\"", text),
                format!("\"{}{}{}\"", style, text, _escape_reset()),
            );
        }
    }
}

pub fn set_random_style(message: &str) {
    let hasher = unsafe { &mut _HASHER };
    message.to_string().hash::<DefaultHasher>(hasher);
    let color_hash: usize = {
        let finish = hasher.finish();
        (finish as usize % 8).try_into().unwrap()
    };
    message
        .chars()
        .rev()
        .collect::<String>()
        .hash::<DefaultHasher>(hasher);
    let bg_hash: usize = {
        let finish = hasher.finish();
        (finish as usize % 8).try_into().unwrap()
    };

    add_color(message, _get_color_by_ordered_number(color_hash));
    add_bg(message, _get_color_by_ordered_number(bg_hash));
}

// #endregion

// #region Key Setters

// #region Macros

macro_rules! push_unique {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::push_unique_key($($rest)*);
    }
}
pub(crate) use push_unique;

macro_rules! push {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::push_key($($rest)*);
    }
}
pub(crate) use push;

macro_rules! push_div {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::push_key_div($($rest)*);
    }
}
pub(crate) use push_div;

macro_rules! pop {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::pop_key($($rest)*);
    }
}
pub(crate) use pop;

macro_rules! pop_unique {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::pop_unique_key($($rest)*);
    }
}
pub(crate) use pop_unique;

// #endregion

pub fn push_unique_key(key: &str) {
    let key = key.to_string();
    unsafe {
        if !_KEYS.contains(&key) {
            _KEYS.push(key);
        }
    }
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

// #endregion

// #region Stylizers

fn _color(color: Color, message: &str) -> String {
    return format!("{}{}{}", _escape_color(color), message, _escape_reset());
}

fn _bg(message: &str, color: Color) -> String {
    return format!("{}{}{}", _escape_bg(color), message, _escape_reset());
}

fn _effect(effect: Effect, message: &str) -> String {
    return format!("{}{}{}", _escape_effect(effect), message, _escape_reset());
}

fn _indent(text: &str, indent: usize) -> String {
    text.replace('\n', &format!("\n{}", "\t".repeat(indent)))
}

fn style_keys(keys: &[&str]) -> Vec<String> {
    let mut styled_keys: Vec<String> = Vec::new();
    for key in keys {
        unsafe {
            if _STYLES.borrow().contains_key(*key) {
                styled_keys.push(_STYLES.borrow().get(*key).unwrap().clone());
            } else {
                styled_keys.push(key.to_string());
            }
        }
    }
    styled_keys
}

fn style_text(message: &str) -> String {
    let message_parts = message.split_whitespace().collect::<Vec<&str>>();
    let mut message_splitters: Vec<(usize, &str)> =
        message.match_indices(|c: char| c.is_whitespace()).collect();
    let mut styled_message = String::new();
    for part in message_parts {
        unsafe {
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
    }
    styled_message
}

// #endregion

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

// #endregion

// #region Styleable String Implementations

pub trait Styleable {
    fn color(&self, color: Color) -> String;
    fn bg(&self, color: Color) -> String;
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
    fn bg(&self, color: Color) -> String {
        _bg(self, color)
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
    fn bg(&self, color: Color) -> String {
        _bg(self, color)
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

static mut _STYLES: LazyCell<RefCell<HashMap<String, String>>> =
    LazyCell::new(|| RefCell::new(HashMap::new()));

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

fn _escape_color(color: Color) -> String {
    return format!("\x1b[{}m", _get_escape_code_for_color(color));
}

fn _escape_bg(color: Color) -> String {
    return format!("\x1b[{}m", _get_escape_code_for_bg(color));
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
