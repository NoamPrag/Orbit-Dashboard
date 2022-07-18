import { Accessor, Component, JSX } from "solid-js";
import "./App.scss";
import { createNetworkTableSignal } from "./network/NetworkTables";

const robotX: Accessor<number> = await createNetworkTableSignal(
  "/Match/Pose/X",
  0
);

const robotY: Accessor<number> = await createNetworkTableSignal(
  "/Match/Pose/Y",
  0
);

const App: Component = (): JSX.Element => {
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
