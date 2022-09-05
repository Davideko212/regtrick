import { writable } from "svelte/store";

export const activePanel = writable(0);
export const ActivePanel = {
    Warning: "Warning",
    Main: "Main",
    Options: "Options",
    Info: "Info"
}

export const activeTheme = writable(2);
export const ActiveTheme = [
    {
        "name": "Undefined",
        "sheet": ""
    },
    {
        "name": "Light",
        "sheet": "smui.css"
    },
    {
        "name": "Dark",
        "sheet": "smui-dark.css"
    },
    {
        "name": "Full Dark",
        "sheet": ""
    }
]

export const explorerWidth = writable(300);
export const explorerOldWidth = writable(0);
export const explorerExtended = writable(true);

export const selectedEntries = writable([]);