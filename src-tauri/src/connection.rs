use std::sync::Mutex;

use nt::{Client, NetworkTables};
use tauri::State;

#[derive(Default)]
pub struct ConnectionState(pub Mutex<Option<NetworkTables<Client>>>);

const CLIENT_NAME: &str = "Orbit-Dashboard";

#[tauri::command]
pub async fn connect(ip: String, conn_state: State<'_, ConnectionState>) -> Result<(), String> {
    let client = NetworkTables::connect(&ip, CLIENT_NAME).await;
    if let Err(e) = client {
        return Err(e.to_string());
    }

    let connected_state: Option<NetworkTables<Client>> = Some(client.unwrap());
    *conn_state.0.lock().unwrap() = connected_state;
    Ok(())
}
