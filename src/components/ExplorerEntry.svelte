<script>
    import { explorerWidth } from '../stores.js';

    let enabled = false;
    export let title = "";

    let width = 300;
    
    explorerWidth.subscribe(value => {
		width = value;
	})
</script>

<main>
    <div id="content" style="--width: {width}px">
        <div id="title">
            <h1>{title}</h1>
        </div>
        <div id="enable">
            <input type="checkbox" id="switch" bind:checked={enabled}>
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

        background-color: red;
        border: 1px;
        border-style: solid;
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