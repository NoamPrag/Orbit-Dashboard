import { invoke } from "@tauri-apps/api";

type EntryValueType = "String" | "Double" | "Boolean"; // TODO: add all types

const tsTypeToEntryType = new Map<string, EntryValueType>();
tsTypeToEntryType.set("string", "String");
tsTypeToEntryType.set("number", "Double");
tsTypeToEntryType.set("boolean", "Boolean");

type EntryValue = string | number | boolean;

type InvokePayload = { type: EntryValueType; content: EntryValue };

const getPayload = (value: EntryValue): InvokePayload => {
  const entryValueType: EntryValueType = tsTypeToEntryType.get(typeof value);
  return { type: entryValueType, content: value };
};

const SET_ENTRY_VALUE_CMD: string = "set_entry_value";
const setEntryValue = async (
  entryName: string,
  entryValue: EntryValue
): Promise<unknown> =>
  await invoke(SET_ENTRY_VALUE_CMD, {
    entryName,
    value: getPayload(entryValue),
  }).catch(console.log);

export type NetworkTableSetter<T extends EntryValue> = (
  value: T
) => Promise<void>;

export const createNetworkTableSetter =
  <T extends EntryValue>(entry: string): NetworkTableSetter<T> =>
  async (value: T): Promise<void> => {
    await setEntryValue(entry, value);
  };
