<script>
    import { explorerWidth } from '../stores.js';
    import Icon from './Icon.svelte';
    import Entries from "../../public/assets/entries.json";

    export let title = "";

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
        background-color: blue;
        aspect-ratio: 1/1;
        margin: 3px;
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