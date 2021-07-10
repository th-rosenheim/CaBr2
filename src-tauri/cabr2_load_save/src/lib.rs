#![allow(clippy::new_without_default)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::upper_case_acronyms)]

mod error;
pub mod handler;
mod types;

#[cfg(feature = "tauri_plugin")]
pub mod plugin;
#[cfg(feature = "webserver")]
pub mod webserver;

#[cfg(feature = "beryllium")]
mod beryllium;
#[cfg(feature = "cabr2")]
mod cabr2;
#[cfg(feature = "pdf")]
mod pdf;
