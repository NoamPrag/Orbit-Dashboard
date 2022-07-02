use nt::EntryValue;

#[derive(Clone)]
pub struct SerializableEntryValue(EntryValue);

impl SerializableEntryValue {
    pub fn wrap(entry_value: EntryValue) -> Self {
        SerializableEntryValue(entry_value)
    }
}

impl serde::ser::Serialize for SerializableEntryValue {
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
