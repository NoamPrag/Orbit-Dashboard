import { Accessor, Component, JSX } from "solid-js";
import "./App.scss";
import {
  createNetworkTableSignal,
  createNetworkTableSetter,
  NetworkTableSetter,
} from "./network";

const setEntry: NetworkTableSetter<number> =
  createNetworkTableSetter("/Example/Entry");

const robotX: Accessor<number> = await createNetworkTableSignal(
  "/Match/Pose/X",
  0
);

const robotY: Accessor<number> = await createNetworkTableSignal(
  "/Match/Pose/Y",
  0
);

const App: Component = (): JSX.Element => {
  setEntry(1);
  return (
    <>
      <div>
        <h1>
          pos: ({robotX().toFixed(2)}, {robotY().toFixed(2)})
        </h1>
      </div>
    </>
  );
};

export default App;
