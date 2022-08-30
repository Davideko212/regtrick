<script>
    import { explorerWidth } from '../stores.js';
    import Icon from './Icon.svelte';
    import Entries from "../../src-tauri/src/entries.json";
    import ExplorerCategory from './ExplorerCategory.svelte';

    let width = 300;
    let oldWidth = 0;
    let extended = true;

    function resizeExplorer(event) {
        if (event.clientX >= 200 && event.clientX <= 450) {
            width = event.clientX;
            explorerWidth.set(width);
        }
    }

    function onMouseDown(event) {
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

        explorerWidth.set(width);
    }
</script>

<main>
    <div id="content" style="--width: {width}px">
        <div id="top">
            {#if extended}
            <input type="text" id="searchbar" style="--width: {width-80}px">
            {/if}
            <div id="expand-container" on:click={toggle_extend}>
                {#if extended}
                    <Icon name={"Minus"}/>
                {:else}
                    <Icon name={"Plus"}/>
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

    #content {
        display: flex;
        flex-direction: column;
        position: fixed;
        top: 0%;
        left: 0%;
        height: 100%;
        width: var(--width);

        background-color: violet;
    }

    #drag {
        position: absolute;
        height: 100%;
        width: 24px;
        margin-left: var(--width);
        cursor: e-resize;

        /*background-color: yellow;*/
    }

    #top {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 22px;

        background-color: aqua;
    }

    #searchbar {
        width: var(--width);
        margin-left: 16px;
    }

    #expand-container {
        padding: 5px;
        aspect-ratio: 1/1;
        cursor: pointer;

        background-color: blueviolet;
    }
</style>