use std::{
    cell::{LazyCell, RefCell},
    collections::HashMap,
};

// #region Loggers

pub const IS_V: bool = cfg!(feature = "v");

#[cfg(feature = "vv")]
pub const IS_VV: bool = true;
#[cfg(not(feature = "vv"))]
pub const IS_VV: bool = false;

#[cfg(feature = "vvv")]
pub const IS_VVV: bool = true;
#[cfg(not(feature = "vvv"))]
pub const IS_VVV: bool = false;

// #region Macros

#[allow(unused_macros)]
macro_rules! log {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::log_info($($rest)*);
    };
}
#[allow(unused_imports)]
pub(crate) use log;

macro_rules! info {
    ($($rest:tt)*) => {
        #[cfg(feature = "v")]
        log::log_info($($rest)*);
    };
}
pub(crate) use info;

macro_rules! vv {
    ($($rest:tt)*) => {
        #[cfg(feature = "vv")]
        log::log_info($($rest)*);
    };
}
pub(crate) use vv;

macro_rules! vvv {
    ($($rest:tt)*) => {
        #[cfg(feature = "vvv")]
        log::log_info($($rest)*);
    };
}
pub(crate) use vvv;

#[allow(unused_macros)]
macro_rules! plain {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::log_info_plain($($rest)*);
    }
}
#[allow(unused_imports)]
pub(crate) use plain;

#[allow(unused_macros)]
macro_rules! warning {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::log_warn($($rest)*);
    }
}
#[allow(unused_imports)]
pub(crate) use warning;

#[allow(unused_macros)]
macro_rules! error {
    ($($rest:tt)*) => {
        #[cfg(feature = "log")]
        log::log_error($($rest)*);
    }
}
#[allow(unused_imports)]
pub(crate) use error;

// macro_rules! ln {
//     () => {
//         #[cfg(feature = "verbose")]
//         log::log_ln();
//     };
// }
// pub(crate) use ln;

// #endregion

fn style_and_log(keys: &[&str], message: &str) {
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
    style_and_log(
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
    style_and_log(
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
    style_and_log(
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

// #endregion

pub fn add_bg(text: &str, bg: Color) {
    add_style(text, &escape_bg(bg));
}

pub fn add_color(text: &str, color: Color) {
    add_style(text, &escape_color(color));
}

pub fn add_effect(text: &str, effect: Effect) {
    add_style(text, &escape_effect(effect));
}

pub fn add_style(text: &str, style: &str) {
    let text = text.to_string();
    unsafe {
        if _STYLES.borrow().contains_key(&text) {
            _STYLES
                .borrow_mut()
                .insert(text.clone(), format!("{}{}{}", style, text, escape_reset()));
            _STYLES.borrow_mut().insert(
                format!("\"{}\"", text),
                format!("\"{}{}{}\"", style, text, escape_reset()),
            );
        } else {
            _STYLES
                .borrow_mut()
                .insert(text.clone(), format!("{}{}{}", style, text, escape_reset()));
            _STYLES.borrow_mut().insert(
                format!("\"{}\"", text),
                format!("\"{}{}{}\"", style, text, escape_reset()),
            );
        }
    }
}

pub fn set_random_style(message: &str) {
    let color = get_random_color(message);
    let bg = get_random_color(&format!("bg{}", message));

    add_color(message, color);
    add_bg(message, bg);
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

use super::ansi::{
    escape_bg, escape_color, escape_effect, escape_reset, get_random_color, Color, Effect,
    Styleable,
};

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

    let first_is_ws = message.starts_with(|c: char| c.is_whitespace());
    if first_is_ws {
        let mut next_splitter = message_splitters.remove(0);
        styled_message.push_str(next_splitter.1);
        while (!message_splitters.is_empty())
            && ((message_splitters.first().unwrap().0 - next_splitter.0) == 1)
        {
            next_splitter = message_splitters.remove(0);
            styled_message.push_str(next_splitter.1);
        }
    }

    for part in message_parts {
        unsafe {
            if _STYLES.borrow().contains_key(part) {
                styled_message.push_str(_STYLES.borrow().get(part).unwrap());
            } else {
                styled_message.push_str(part);
            }
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
    styled_message
}

// #endregion

// #region Internal

static mut _KEYS: Vec<String> = Vec::new();

static mut _STYLES: LazyCell<RefCell<HashMap<String, String>>> =
    LazyCell::new(|| RefCell::new(HashMap::new()));

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
