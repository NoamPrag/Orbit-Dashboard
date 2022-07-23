use async_mutex::Mutex;

use nt::{Client, NetworkTables};
use tauri::State;

#[derive(Default)]
pub struct ConnectionState(pub Mutex<Option<NetworkTables<Client>>>);

const CLIENT_NAME: &str = "Orbit-Dashboard";

#[tauri::command]
pub async fn connect(ip: String, conn_state: State<'_, ConnectionState>) -> Result<(), String> {
    let conn_result = NetworkTables::connect(&ip, CLIENT_NAME).await;

    match conn_result {
        Ok(client) => {
            // If lock doesn't work: try https://docs.rs/futures/0.3.21/futures/lock/struct.Mutex.html
            *conn_state.0.lock().await = Some(client);
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}
