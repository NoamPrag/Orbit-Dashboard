/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";

const ROOT: HTMLElement = document.getElementById("root");

render(() => <App />, ROOT);
