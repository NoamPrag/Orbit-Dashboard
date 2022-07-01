#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[derive(Default)]
struct ConnState(Mutex<Option<NetworkTables<Client>>>);

impl ConnState {
    // TODO better name for function
    async fn connect(ip: &str) -> nt::Result<NetworkTables<Client>> {
        // ? client name
        let client: NetworkTables<Client> = NetworkTables::connect(ip, "OrbitDashboard").await?;

        client.add_connection_callback(ConnectionCallbackType::ClientDisconnected, |_| {
            println!("Client connected!");
        });
        Ok(client)
    }
}

use std::sync::Mutex;

use nt::{CallbackType, Client, ConnectionCallbackType, EntryData, EntryValue, NetworkTables};
use tauri::{window::Window, State};

#[derive(Clone)]
struct EntryValueWrapper(EntryValue);

impl serde::ser::Serialize for EntryValueWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            EntryValue::Boolean(value) => serializer.serialize_bool(*value),
            EntryValue::Double(value) => serializer.serialize_f64(*value),
            EntryValue::String(value) => serializer.serialize_str(&value),
            EntryValue::RawData(bytes) => serializer.collect_seq(bytes.iter()),
            EntryValue::BooleanArray(values) => serializer.collect_seq(values.iter()),
            EntryValue::DoubleArray(values) => serializer.collect_seq(values.iter()),
            EntryValue::StringArray(values) => serializer.collect_seq(values.iter()),
            EntryValue::RpcDefinition(_) => {
                serializer.serialize_unit_variant("RpcDefinition", 0, "V0")
            }
        }
    }
}

fn read_entry_value(entry_name: &str, client: &NetworkTables<Client>) -> Option<EntryValue> {
    for (_, entry) in client.entries() {
        if entry.name == entry_name {
            return Some(entry.value);
        }
    }
    None
}

#[tauri::command]
fn listen_to_entry<'a>(
    entry_name: String,
    window: Window,
    conn_state: State<'_, ConnState>,
    // TODO create a struct to hold the Ok variant of the result
) -> Result<(String, Option<EntryValueWrapper>), String> {
    match conn_state.0.lock().unwrap().as_mut() {
        Some(client) => {
            let entry_clone: String = entry_name.clone(); // TODO don't clone string

            client.add_callback(CallbackType::Update, move |entry_data: &EntryData| {
                let value: EntryValueWrapper = EntryValueWrapper(entry_data.value.clone()); // TODO figure out a way to not clone values
                if let Err(err) = window.emit(&entry_name, value) {
                    println!("{}", err); // TODO understand window emit error and do something appropriate
                }
            });

            let entry_value: Option<EntryValueWrapper> = read_entry_value(&entry_clone, client)
                .map(|value: EntryValue| EntryValueWrapper(value));

            Ok((entry_clone, entry_value))
        }

        None => Err(String::from("Not connected to robot")),
    }
}

#[tauri::command]
async fn connect(ip: String, conn_state: State<'_, ConnState>) -> Result<(), ()> {
    // TODO convert nt::Error to string and return it in case of a failure
    let connected_state: Option<NetworkTables<Client>> =
        Some(ConnState::connect(&ip).await.unwrap());
    *conn_state.0.lock().unwrap() = connected_state;
    Ok(())
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
        .manage(ConnState::default())
        .invoke_handler(tauri::generate_handler![connect, listen_to_entry])
        .run(context)
        .expect("error while running tauri application");
}
