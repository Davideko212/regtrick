import { writable } from "svelte/store";

export const activePanel = writable(0);

export const ActivePanel = {
    Warning: "Warning",
    Main: "Main"
}

export const explorerWidth = writable(300);