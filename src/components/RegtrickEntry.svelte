<script>
    import Icon from '../components/Icon.svelte';
    import Entries from "../../public/assets/entries.json";

    export let enabled = false;
    export let title = "";
    export let active = "";
    export let description = "";

    let extended = false;

    function toggle_extend() {
        extended = !extended;
    }
</script>

<main>
    <div id="content">
        <div id="wrapper">
            <div id="title">
                <div id="arrow">
                    <div id="arrow-container" on:click={toggle_extend}>
                        {#if extended}
                            <Icon name={"Arrow_down"}/>
                        {:else}
                            <Icon name={"Arrow_up"}/>
                        {/if}
                    </div>
                </div>
                <h1>{title}</h1>
            </div>
            {#if extended}
                <div id="entry">
                    <div id="important">
                        <div id="enable">
                            <h2>Enable:</h2>
                            <input type="checkbox" id="switch" bind:checked={enabled}>
                        </div>
                        <div id="active">
                            <h2>Active: {active}</h2>
                        </div>
                    </div>
                    <p>{description}</p>
                </div>
            {/if}
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
        border: 5px;
    }

    #entry {
        display: flex;
        flex-direction: column;
        margin-left: 26px;
        gap: 10px;
        border-top: 2px;
        border-style: solid;
        border-color: lightgrey;
        padding-top: 10px;
    }

    #enable {
        display: flex;
        flex-direction: row;
        gap: 10px;
    }

    #switch {
        width: 20px;
        height: 20px;
        align-self: center;
        cursor: pointer;
    }

    #important {
        display: flex;
        flex-direction: column;
        gap: 3px;
    }

    #title {
        margin-top: 2vh;
        margin-bottom: 5px;
        display: flex;
        flex-direction: row;
        gap: 8px;
    }

    #arrow {
        align-self: center;
        margin-top: 10px;
    }

    #arrow-container {
        cursor: pointer;
    }

    h1 {
        margin: 0;
    }
</style>