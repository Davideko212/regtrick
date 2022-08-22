<script>
    import { explorerWidth } from '../stores.js';
    import Icon from './Icon.svelte';
    import ExplorerEntry from './ExplorerEntry.svelte';
    import RegtrickEntry from './RegtrickEntry.svelte';

    export let title = "";
    export let values;

    let width = 300;
    let extended = false;
    
    explorerWidth.subscribe(value => {
		width = value;
	})

    function toggle_extend() {
        extended = !extended;
    }
</script>

<main>
    <div id="content" style="--width: {width}px">
        <div id="expand-container" on:click={toggle_extend}>
            {#if extended}
                <Icon name={"Minus"} width={19} height={19}/>
            {:else}
                <Icon name={"Plus"} width={19} height={19}/>
            {/if}
        </div>
        <div id="title">
            <h1>{title}</h1>
        </div>
    </div>
    <div id="entries" style="--width: {width}px">
        {#if extended}
            {#each values as entry}
                <ExplorerEntry title={entry.title} enabled={entry.enabled}/>
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
        gap: 3px;
        height: 25px;

        background-color: coral;
        border: 1px;
        border-style: solid;
    }

    #expand-container {
        aspect-ratio: 1/1;
        margin: 3px;
        cursor: pointer;

        background-color: blue;
    }

    h1 {
        font-size: 16px;
    }
</style>