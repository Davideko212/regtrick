import { writable } from "svelte/store";

export const activePanel = writable(0);

export const ActivePanel = {
    Warning: "Warning",
    Main: "Main"
}

export const Icons = {
    Arrow_up: {
        path: "assets/icons/arrow_up.svg",
        alt: "Chevron arrow pointing up",
        width: 16,
        height: 16
    },
    Arrow_down: {
        path: "assets/icons/arrow_down.svg",
        alt: "Chevron arrow pointing down",
        width: 16,
        height: 16
    }
}