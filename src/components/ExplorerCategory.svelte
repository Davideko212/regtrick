<script>
    import { explorerWidth, selectedEntries } from '../stores.js';
    import Icon from './Icon.svelte';
    import ExplorerEntry from './ExplorerEntry.svelte';

    export let title = "";
    export let values;

    let width = 300;
    let color;
    let extended = false;
    let selected = false;
    let highlighted = false;

    $: if (selected) {
        color = "#555555";
    } else if (highlighted) {
        color = "#424242";
    } else {
        color = "#282828";
    }
    
    explorerWidth.subscribe(value => {
		width = value;
	})

    selectedEntries.subscribe(selectedValues => {
        if (selectedValues != values && selectedValues.length != 0) {
            selected = false;
        }
    })

    function toggle_extend() {
        extended = !extended;
    }

    function toggle_select() {
        selected = !selected;
        if (selected) {
            selectedEntries.set(values);
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
    <div id="content" style="--color: {color}" on:mouseover={toggle_highlight} on:mouseout={toggle_highlight}>
        <div id="expand-container" on:click={toggle_extend}>
            {#if extended}
                <Icon name={"remove"} size={20}/>
            {:else}
                <Icon name={"add"} size={20}/>
            {/if}
        </div>
        <div id="select-container" on:click={toggle_select}>
            <div id="title">
                <h1>{title}</h1>
            </div>
        </div>
    </div>
    <div id="entries">
        {#if extended}
            {#each values as entry}
                <ExplorerEntry title={entry.title} value={entry}/>
            {/each}
        {/if}
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
        height: 20px;

        background-color: var(--color);
        border: 1px;
        border-style: solid;
    }

    #expand-container {
        aspect-ratio: 1/1;
        margin: 3px;
        margin-left: 0;
        cursor: pointer;
    }

    #select-container {
        width: 100%;
        cursor: pointer;
    }

    h1 {
        font-size: 16px;
    }
</style>