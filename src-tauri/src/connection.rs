use std::sync::Mutex;

use nt::{Client, NetworkTables};
use tauri::State;

#[derive(Default)]
pub struct ConnectionState(pub Mutex<Option<NetworkTables<Client>>>);

const CLIENT_NAME: &str = "Orbit-Dashboard";

#[tauri::command]
pub async fn connect(ip: String, conn_state: State<'_, ConnectionState>) -> Result<(), ()> {
    // TODO: convert nt::Error to string and return it in case of a failure
    let connected_state: Option<NetworkTables<Client>> =
        Some(NetworkTables::connect(&ip, CLIENT_NAME).await.unwrap());
    *conn_state.0.lock().unwrap() = connected_state;
    Ok(())
}
