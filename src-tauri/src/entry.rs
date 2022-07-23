use nt::{Client, EntryData, EntryValue, NetworkTables};

pub fn get_entry_id(entry_name: &str, client: &NetworkTables<Client>) -> Option<u16> {
    client
        .entries()
        .iter()
        .find(|(_, entry_data)| entry_data.name == entry_name)
        .map(|(id, _)| *id)
}

pub fn get_entry_data(entry_name: &str, client: &NetworkTables<Client>) -> Option<EntryData> {
    get_entry_id(entry_name, client).map(|id: u16| client.get_entry(id).value())
}

pub fn read_entry_value(entry_name: &str, client: &NetworkTables<Client>) -> Option<EntryValue> {
    get_entry_data(entry_name, client).map(|entry: EntryData| entry.value)
}
