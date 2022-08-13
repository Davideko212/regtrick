<script>
    import { explorerWidth } from '../stores.js';
import Icon from './Icon.svelte';

    let width = 300;

    function resizeExplorer(event) {
        if (event.clientX >= 150 && event.clientX <= 450) {
            width = event.clientX;
            explorerWidth.set(width);
        }
    }

    function onMouseDown(event) {
        addEventListener("mousemove", resizeExplorer);
        addEventListener("mouseup", onMouseUp);
    }

    function onMouseUp() {
        removeEventListener("mousemove", resizeExplorer);
        removeEventListener("mouseup", onMouseUp);
    }
</script>

<main>
    <div id="content" style="--width: {width}px">
        <div id="top">
            <input type="text" id="searchbar" style="--width: {width-10}px">
            <div id="expand-container">
                <Icon name={"X"}/>
            </div>
        </div>
    </div>
    <div id="drag" on:mousedown={onMouseDown} style="--width: {width-12}px">
        
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
        flex-direction: column;
        position: absolute;
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
        padding: 8px;
        padding-left: 16px;
        gap: 20px;

        background-color: aqua;
    }

    #searchbar {
        width: var(--width);
    }

    #expand-container {
        align-self: flex-end;

        background-color: blueviolet;
    }
</style>