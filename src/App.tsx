import { Component, createSignal } from "solid-js";
import "./App.scss";

const App: Component = () => {
  const [counter, setCounter] = createSignal(0);
  setInterval(setCounter, 1000, (c: number): number => c + 1);

  return (
    <>
      <div>
        <h1>{counter()}</h1>
      </div>
    </>
  );
};

export default App;
