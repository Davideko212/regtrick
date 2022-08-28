<script>
    import { explorerWidth, selectedEntries } from '../stores.js';

    export let title = "";
    export let value;

    let width = 300;
    let color;
    let enabled = false;
    let selected = false;
    
    $: if (selected) {
        color = "#ff6161";
    } else {
        color = "#ff0000";
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

    function toggle_select() {
        selected = !selected;
        if (selected) {
            selectedEntries.set([value]);
        } else {
            selectedEntries.set([]);
        }
    }
</script>

<main>
    <div id="content" style="--width: {width}px; --color: {color}" on:click={toggle_select}>
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
        height: 25px;

        background-color: var(--color);
        border: 1px;
        border-style: solid;

        cursor: pointer;
    }

    #switch {
        aspect-ratio: 1/1;
        width: 14px;
        margin-left: 5px;
    }

    h1 {
        font-size: 16px;
    }
</style>