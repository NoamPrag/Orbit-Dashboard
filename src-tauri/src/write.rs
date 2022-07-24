use nt::{EntryData, EntryValue};
use tauri::State;

use crate::connection::ConnectionState;
use crate::entry::get_entry_id;
use crate::serializable_entry_value::SerializableEntryValue;

#[tauri::command]
pub async fn set_entry_value(
    entry_name: String,
    value: SerializableEntryValue,
    conn_state: State<'_, ConnectionState>,
) -> Result<(), String> {
    match conn_state.0.lock().await.as_mut() {
        Some(client) => {
            let value: EntryValue = value.into();
            match get_entry_id(&entry_name, client) {
                // If an entry id exists, an entry with the given name already exists
                Some(id) => client.update_entry(id, value),
                None => {
                    let entry_creation_result = client
                        .create_entry(EntryData::new(entry_name, 0, value))
                        .await;

                    // TODO: handle error more properly
                    if let Err(err) = entry_creation_result {
                        return Err(err.to_string());
                    }
                }
            };
            Ok(())
        }
        None => Err(String::from("Not connected to robot")),
    }
}
