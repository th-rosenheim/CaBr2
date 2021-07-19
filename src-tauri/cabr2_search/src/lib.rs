#![allow(clippy::new_without_default)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::upper_case_acronyms)]

mod error;
pub mod handler;
mod types;

#[cfg(feature = "gestis")]
mod gestis;

#[cfg(feature = "tauri_plugin")]
pub mod plugin;
#[cfg(feature = "webserver")]
pub mod webserver;
