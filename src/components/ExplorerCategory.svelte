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

    $: if (selected) {
        color = "#fdab8e";
    } else {
        color = "#ff7f50";
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
</script>

<main>
    <div id="content" style="--color: {color}">
        <div id="expand-container" on:click={toggle_extend}>
            {#if extended}
                <Icon name={"Minus"} width={19} height={19}/>
            {:else}
                <Icon name={"Plus"} width={19} height={19}/>
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
        height: 25px;

        background-color: var(--color);
        border: 1px;
        border-style: solid;
    }

    #expand-container {
        aspect-ratio: 1/1;
        margin: 3px;
        cursor: pointer;

        background-color: blue;
    }

    #select-container {
        width: 100%;
        cursor: pointer;
    }

    h1 {
        font-size: 16px;
    }
</style>