#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod entry;

mod read;
mod serializable_entry_value;

mod write;

mod connection;
use connection::ConnectionState;

fn main() {
    let context = tauri::generate_context!();

    let menu: tauri::Menu = if cfg!(target_os = "macos") {
        tauri::Menu::os_default(&context.package_info().name)
    } else {
        tauri::Menu::default()
    };

    tauri::Builder::default()
        .menu(menu)
        .manage(ConnectionState::default())
        .invoke_handler(tauri::generate_handler![
            connection::connect,
            read::listen_to_entry,
            write::set_entry_value,
        ])
        .run(context)
        .expect("error while running tauri application");
}
