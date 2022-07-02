#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod serializable_entry_value;
use serializable_entry_value::SerializableEntryValue;

mod connection;
use connection::ConnectionState;

use nt::{CallbackType, Client, EntryData, EntryValue, NetworkTables};
use tauri::{window::Window, State};

fn read_entry_value(entry_name: &str, client: &NetworkTables<Client>) -> Option<EntryValue> {
    for (_, entry) in client.entries() {
        if entry.name == entry_name {
            return Some(entry.value);
        }
    }
    None
}

#[tauri::command]
async fn listen_to_entry<'a>(
    entry_name: String,
    window: Window,
    conn_state: State<'_, ConnectionState>,
    // TODO: create a struct to hold the Ok variant of the result
) -> Result<(String, Option<SerializableEntryValue>), String> {
    match conn_state.0.lock().unwrap().as_mut() {
        Some(client) => {
            let entry_name_clone: String = entry_name.clone(); // TODO: don't clone string

            client.add_callback(CallbackType::Update, move |entry_data: &EntryData| {
                let serializable_value: SerializableEntryValue =
                    SerializableEntryValue::wrap(entry_data.value.clone()); // TODO: figure out a way to not clone values
                if let Err(err) = window.emit(&entry_name_clone, serializable_value) {
                    println!("{}", err); // TODO: understand window emit error and do something appropriate
                }
            });

            let entry_value: Option<SerializableEntryValue> =
                read_entry_value(&entry_name, client).map(SerializableEntryValue::wrap);

            Ok((entry_name, entry_value))
        }

        None => Err(String::from("Not connected to robot")),
    }
}

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
            listen_to_entry
        ])
        .run(context)
        .expect("error while running tauri application");
}
