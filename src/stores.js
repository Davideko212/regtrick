import { writable } from "svelte/store";

export const activePanel = writable(0);

export const ActivePanel = {
    Warning: "Warning",
    Main: "Main",
    Options: "Options",
    Info: "Info"
}

export const explorerWidth = writable(300);
export const explorerOldWidth = writable(0);
export const explorerExtended = writable(true);

export const selectedEntries = writable([]);