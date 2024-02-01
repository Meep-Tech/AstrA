use std::{
    cell::{LazyCell, OnceCell},
    sync::{Arc, LazyLock, Mutex},
};

static _ACTIVE_CHANNELS: Mutex<LazyCell<Vec<String>>> = Mutex::new(LazyCell::new(|| Vec::new()));
static ACTIVE_CHANNELS: LazyLock<Vec<String>> =
    LazyLock::new(|| _ACTIVE_CHANNELS.lock().unwrap().clone());

macro_rules! log {
    () => {};
}

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
        if (chans.is_empty() || chans.contains("log") || chans.contains("info") || chans.contains("warn") || chans.contains("err") || active_channels.iter().any(|c| chans.contains(c))) {
          let keys: Vec<&str> = vec![$($(stringify!($ident),)*)*];
          let message: String = format!($msg, $($args),*);


        }
      }
  };
}
