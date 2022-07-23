use nt::{CallbackType, EntryData};
use tauri::{State, Window};

use crate::connection::ConnectionState;
use crate::entry::read_entry_value;
use crate::serializable_entry_value::SerializableEntryValue;

#[derive(serde::Serialize)]
pub struct NetworkTableListenResult {
    ipc_channel: String,
    initial_value: Option<SerializableEntryValue>,
}

#[tauri::command]
pub async fn listen_to_entry(
    entry_name: String,
    window: Window,
    conn_state: State<'_, ConnectionState>,
) -> Result<NetworkTableListenResult, String> {
    match conn_state.0.lock().await.as_mut() {
        Some(client) => {
            let entry_name_clone: String = entry_name.clone(); // TODO: don't clone string

            client.add_callback(CallbackType::Update, move |entry_data: &EntryData| {
                if &entry_data.name != &entry_name_clone {
                    // TODO: use only one update callback that sends values to corresponding channels, based on entries' names
                    return; // Don't send values of other entries
                }

                let serializable_value: SerializableEntryValue =
                    SerializableEntryValue::wrap(entry_data.value.clone()); // TODO: figure out a way to not clone values

                window
                    .emit(&entry_name_clone, serializable_value)
                    .unwrap_or_else(|err: tauri::Error| println!("{}", err));
            });

            let entry_value: Option<SerializableEntryValue> =
                read_entry_value(&entry_name, client).map(SerializableEntryValue::wrap);

            Ok(NetworkTableListenResult {
                ipc_channel: entry_name,
                initial_value: entry_value,
            })
        }

        None => Err(String::from("Not connected to robot")),
    }
}
