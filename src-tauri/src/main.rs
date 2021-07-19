#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

#[cfg(feature = "tauri_app")]
mod tauri_app;
#[cfg(feature = "webserver")]
mod webserver;

fn main() {
  #[cfg(feature = "tauri_app")]
  tauri_app::main();

  #[cfg(feature = "webserver")]
  webserver::main();
}

// Validity checks for features

#[cfg(not(any(feature = "tauri_app", feature = "webserver")))]
compile_error!("you must specify one of these features: 'tauri_app', 'webserver'!");

#[cfg(all(feature = "tauri_app", feature = "webserver"))]
compile_error!("you can only use one of these features: 'tauri_app', 'webserver'!");
