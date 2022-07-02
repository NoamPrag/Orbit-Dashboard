import { Event, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { Accessor, createSignal } from "solid-js";

interface NetworkTableListenResult<T> {
  readonly ipcChannel: string;
  readonly initialValue?: T;
}

type ListenCommandResponse<T> = [string, T];

const parseResponse = <T>(
  response: ListenCommandResponse<T>
): NetworkTableListenResult<T> => ({
  ipcChannel: response[0],
  initialValue: response[1],
});

const reportTypeMismatch = (
  expectedValue: unknown,
  actualValue: unknown
): void => {
  console.error(
    `Mismatched types! type ${typeof expectedValue} does not correlate with the type of the network table value: ${typeof actualValue}`
  );
};

const LISTEN_TO_ENTRY_CMD: string = "listen_to_entry";

export const createNetworkTableSignal = async <T>(
  entry: string,
  defaultValue: T
): Promise<Accessor<T>> => {
  const [value, setValue] = createSignal<T>();

  await invoke(LISTEN_TO_ENTRY_CMD, { entryName: entry })
    .then(parseResponse)
    .then((result: NetworkTableListenResult<T>): void => {
      // Check if network table exists
      const hasInitialValue: boolean = result.initialValue !== undefined;

      // Check if types correlate
      if (
        hasInitialValue &&
        typeof result.initialValue !== typeof defaultValue
      ) {
        reportTypeMismatch(defaultValue, result.initialValue);
        return;
      }

      // Setting current value
      setValue((): T => (hasInitialValue ? result.initialValue : defaultValue));

      // Listening for future changes
      listen(result.ipcChannel, (ipcEvent: Event<T>): void => {
        // ? Don't update value when types don't match?
        setValue((): T => ipcEvent.payload);
      });
    })
    .catch(console.error);

  return value;
};
