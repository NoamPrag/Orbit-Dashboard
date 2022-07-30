use nt::{EntryValue, RpcDefinition::V0};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub enum SerializableEntryValue {
    Boolean(bool),
    Double(f64),
    String(String),
    RawData(Vec<u8>),
    BooleanArray(Vec<bool>),
    DoubleArray(Vec<f64>),
    StringArray(Vec<String>),
    RpcDefinition,
}

impl From<EntryValue> for SerializableEntryValue {
    fn from(value: EntryValue) -> Self {
        match value {
            EntryValue::Boolean(value) => SerializableEntryValue::Boolean(value),
            EntryValue::Double(value) => SerializableEntryValue::Double(value),
            EntryValue::String(value) => SerializableEntryValue::String(value),
            EntryValue::RawData(bytes) => SerializableEntryValue::RawData(bytes),
            EntryValue::BooleanArray(values) => SerializableEntryValue::BooleanArray(values),
            EntryValue::DoubleArray(values) => SerializableEntryValue::DoubleArray(values),
            EntryValue::StringArray(values) => SerializableEntryValue::StringArray(values),
            EntryValue::RpcDefinition(_) => SerializableEntryValue::RpcDefinition,
        }
    }
}

impl Into<EntryValue> for SerializableEntryValue {
    fn into(self) -> EntryValue {
        match self {
            SerializableEntryValue::Boolean(value) => EntryValue::Boolean(value),
            SerializableEntryValue::Double(value) => EntryValue::Double(value),
            SerializableEntryValue::String(value) => EntryValue::String(value),
            SerializableEntryValue::RawData(bytes) => EntryValue::RawData(bytes),
            SerializableEntryValue::BooleanArray(values) => EntryValue::BooleanArray(values),
            SerializableEntryValue::DoubleArray(values) => EntryValue::DoubleArray(values),
            SerializableEntryValue::StringArray(values) => EntryValue::StringArray(values),
            SerializableEntryValue::RpcDefinition => EntryValue::RpcDefinition(V0),
        }
    }
}
