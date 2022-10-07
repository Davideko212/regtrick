<script>
    import { explorerWidth, selectedEntries } from '../stores.js';
    import { invoke } from "@tauri-apps/api";

    export let title = "";
    export let value;

    let width = 300;
    let filter;
    let enabled = false;
    let selected = false;
    let highlighted = false;
    
    $: if (selected) {
        filter = "200%";
    } else if (highlighted) {
        filter = "150%";
    } else {
        filter = "100%";
    }

    explorerWidth.subscribe(value => {
		width = value;
	})

    selectedEntries.subscribe(selectedValues => {
        // For some reason these arrays have to be stringfied first to be compared correctly but the others dont... idk why
        if (JSON.stringify(selectedValues) != JSON.stringify([value]) && selectedValues.length != 0) {
            selected = false;
        }
    })

    async function fetch_enabled() {
        if (await invoke("get_dword", { hkey: value.reg.hkey, path: value.reg.path, key: value.reg.key }) == value.reg.enabledValue) {
            enabled = true;
        }
    }
    fetch_enabled();

    function toggle_select() {
        selected = !selected;
        if (selected) {
            selectedEntries.set([value]);
        } else {
            selectedEntries.set([]);
        }
    }

    function toggle_highlight() {
        highlighted = !highlighted;
    }
</script>

<main>
    <!-- svelte-ignore a11y-mouse-events-have-key-events -->
    <div id="content" style="--width: {width}px; --filter: {filter}" on:mouseover={toggle_highlight} on:mouseout={toggle_highlight} on:click={toggle_select}>
        <div id="title">
            <h1>{title}</h1>
        </div>
        <div id="enable">
            <input type="checkbox" id="switch" disabled bind:checked={enabled}>
        </div>
    </div>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        border: 0;
    }

    #content {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 3px;
        height: 24px;
        text-decoration: none;
        color: inherit;
        filter: brightness(var(--filter));

        cursor: pointer;
    }

    #switch {
        aspect-ratio: 1/1;
        width: 14px;
        margin-left: 5px;
        margin-right: 5px;
    }

    #title {
        display: inline-block;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: "...";
    }

    h1 {
        font-size: 12px;
    }
</style>