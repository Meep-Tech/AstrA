use std::{
    cell::{LazyCell, OnceCell},
    sync::{Arc, LazyLock, Mutex},
};

pub enum Prefix {
    Info,
    Warn,
    Err,
}

static mut _KEYS: Vec<String> = Vec::new();

static _ACTIVE_CHANNELS: Mutex<LazyCell<Vec<String>>> = Mutex::new(LazyCell::new(|| Vec::new()));
static ACTIVE_CHANNELS: LazyLock<Vec<String>> =
    LazyLock::new(|| _ACTIVE_CHANNELS.lock().unwrap().clone());

macro_rules! push {
    ($key:expr) => {
        #[cfg(feature = "log")]
        unsafe {
            _KEYS.push($key);
        }
    };
}

pub(crate) use push;

macro_rules! log {
    (info:
      $($(#$chans:ident $(,)?)+)?
      $([$($keys:ident $(,)?)*])?
      $msg:literal,
      $($args:expr),*
    ) => {
      #[cfg(feature = "log")]
      info!(
        $($(#$chans),*)?
        $([$($keys),*])?
        $msg,
        $($args),*
      );
    };
    (warn:
      $($(#$chans:ident $(,)?)+)?
      $([$($keys:ident $(,)?)*])?
      $msg:literal,
      $($args:expr),*
    ) => {
      #[cfg(feature = "log")]
      warn!(
        $($(#$chans),*)?
        $([$($keys),*])?
        $msg,
        $($args),*
      );
    };
    (err:
      $($(#$chans:ident $(,)?)+)?
      $([$($keys:ident $(,)?)*])?
      $msg:literal,
      $($args:expr),*
    ) => {
      #[cfg(feature = "log")]
      error!(
        $($(#$chans),*)?
        $([$($keys),*])?
        $msg,
        $($args),*
      );
    };
}

pub(crate) use log;

macro_rules! info {
    (
      $($(#$chans:ident $(,)?)+)?
      $([$($keys:ident $(,)?)*])?
      $msg:literal,
      $($args:expr),*
  ) => {
      #[cfg(feature = "log")]
      {
        let chans: Vec<&str> = vec![$($(stringify!($chans),)*)*].into_iter().map(|s| s.trim_start_matches("#").to_string()).collect();
        let active_channels = get_active_channels();
        if (chans.is_empty() || chans.contains("*") || chans.contains("info") || active_channels.iter().any(|c| chans.contains(c))) {
          let keys: Vec<&str> = logs::curr_keys(&[$($(stringify!($ident),)*)*]);
          let message: String = format!($msg, $($args),*);

          println!("[{}]: {}", keys.join("]["), message);
        }
      }
  };
}

pub(crate) use info;

macro_rules! warn {
    (
      $($(#$chans:ident $(,)?)+)?
      $([$($keys:ident $(,)?)*])?
      $msg:literal,
      $($args:expr),*
  ) => {
      #[cfg(feature = "log")]
      {
        let chans: Vec<&str> = vec![$($(stringify!($chans),)*)*].into_iter().map(|s| s.trim_start_matches("#").to_string()).collect();
        let active_channels = get_active_channels();
        if (chans.is_empty() || chans.contains("*") || chans.contains("warn") || active_channels.iter().any(|c| chans.contains(c))) {
          let keys: Vec<&str> = logs::curr_keys(&[$($(stringify!($ident),)*)*]);
          let message: String = format!($msg, $($args),*);

          println!("[{}]: {}", keys.join("]["), message);
        }
      }
  };
}

pub(crate) use warn;

macro_rules! err {
    (
      $($(#$chans:ident $(,)?)+)?
      $([$($keys:ident $(,)?)*])?
      $msg:literal,
      $($args:expr),*
  ) => {
      #[cfg(feature = "log")]
      {
        let chans: Vec<&str> = vec![$($(stringify!($chans),)*)*].into_iter().map(|s| s.trim_start_matches("#").to_string()).collect();
        let active_channels = get_active_channels();
        if (chans.is_empty() || chans.contains("*") || chans.contains("err") || active_channels.iter().any(|c| chans.contains(c))) {
          let keys: Vec<&str> = logs::curr_keys(&[$($(stringify!($ident),)*)*]);
          let message: String = format!($msg, $($args),*);

          println!("[{}]: {}", keys.join("]["), message);
        }
      }
  };
}

pub(crate) use err;

pub fn curr_keys(local: &[&str], prefix: Prefix) -> Vec<String> {
    let mut keys = vec![match prefix {
        Prefix::Info => "*".color(Color::Blue),
        Prefix::Warn => "~".color(Color::Yellow),
        Prefix::Err => "!!".color(Color::Red),
    }];
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
