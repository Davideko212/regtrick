<script>
    import { explorerExtended, explorerWidth, explorerOldWidth } from '../stores.js';
    import Icon from './Icon.svelte';
    import Entries from "../../src-tauri/src/entries.json";
    import ExplorerCategory from './ExplorerCategory.svelte';

    let width;
    let oldWidth;
    let extended;
    $: explorerExtended.set(extended);
    $: explorerWidth.set(width);
    $: explorerOldWidth.set(oldWidth);

    explorerWidth.subscribe(value => {
		width = value;
	})

    explorerOldWidth.subscribe(value => {
		oldWidth = value;
	})

    explorerExtended.subscribe(value => {
		extended = value;
	})

    function resizeExplorer(event) {
        if (event.clientX >= 200 && event.clientX <= 450) {
            width = event.clientX;
        }
    }

    function onMouseDown() {
        if (extended) {
            addEventListener("mousemove", resizeExplorer);
            addEventListener("mouseup", onMouseUp);
        }
    }

    function onMouseUp() {
        removeEventListener("mousemove", resizeExplorer);
        removeEventListener("mouseup", onMouseUp);
    }

    function toggle_extend() {
        extended = !extended;
        if (extended) {
            width = oldWidth;
        } else {
            oldWidth = width;
            width = 42;
        }
    }
</script>

<main>
    <div id="window" style="--width: {width}px">
        <div id="top">
            {#if extended}
            <input type="text" id="searchbar" style="--width: {width-80}px">
            {/if}
            <div id="expand-container" on:click={toggle_extend}>
                {#if extended}
                    <Icon name={"remove"} size={42}/>
                {:else}
                    <Icon name={"add"} size={42}/>
                {/if}
            </div>
        </div>
        {#if extended}
        <div id="entries">
            {#each Entries as category}
                <ExplorerCategory title={category.name} values={category.entries}/>
            {/each}
        </div>
        {/if}
    </div>
    <div id="drag" on:mousedown={onMouseDown} style="--width: {width-12}px"/>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        border: 0;
    }

    #window {
        display: flex;
        flex-direction: column;
        position: fixed;
        top: 0%;
        left: 0%;
        height: 100%;
        width: var(--width);
    }

    #drag {
        position: absolute;
        height: 100%;
        width: 24px;
        margin-left: var(--width);
        cursor: e-resize;
    }

    #top {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 22px;
    }

    #searchbar {
        width: var(--width);
        margin-left: 16px;
    }

    #expand-container {
        aspect-ratio: 1/1;
        cursor: pointer;
    }
</style>