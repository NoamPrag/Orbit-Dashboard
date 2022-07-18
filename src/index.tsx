/* @refresh reload */
import { invoke } from "@tauri-apps/api/tauri";
import { render } from "solid-js/web";

import App from "./App";

// TODO ip should be picked by user
const ROBOT_IP: string = "10.16.90.2:1735";
invoke("connect", { ip: ROBOT_IP }).catch(console.error);

const ROOT: HTMLElement = document.getElementById("root");

render(() => <App />, ROOT);
