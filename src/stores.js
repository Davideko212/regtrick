import { writable } from "svelte/store";

export const activePanel = writable(0);

export const ActivePanel = {
    Warning: "Warning",
    Main: "Main"
}

export const explorerWidth = writable(300);

export const Icons = {
    Arrow_up: {
        path: "arrow_up.svg",
        alt: "Chevron arrow pointing up",
        width: 16,
        height: 16
    },
    Arrow_down: {
        path: "arrow_down.svg",
        alt: "Chevron arrow pointing down",
        width: 16,
        height: 16
    },
    Cog: {
        path: "cog.svg",
        alt: "Cog/Gear Icon",
        width: 32,
        height: 32
    },
    X: {
        path: "x.svg",
        alt: "X Icon",
        width: 32,
        height: 32
    },
    Info: {
        path: "info.svg",
        alt: "Information Icon",
        width: 32,
        height: 32
    },
    Plus: {
        path: "plus.svg",
        alt: "Plus Icon",
        width: 32,
        height: 32
    },
    Minus: {
        path: "minus.svg",
        alt: "Minus Icon",
        width: 32,
        height: 32
    },
}