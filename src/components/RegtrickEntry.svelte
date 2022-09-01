<script>
    import Icon from '../components/Icon.svelte';
    import { invoke } from "@tauri-apps/api";

    let enabled = false;
    let extended = false;

    export let title = "";
    export let active = "";
    export let description = "";
    export let reg = {};
    
    function toggle_extend() {
        extended = !extended;
    }

    async function fetch_enabled() {
        if (await invoke("get_dword", { hkey: reg.hkey, path: reg.path, key: reg.key }) == reg.enabledValue) {
            enabled = true;
        }
    }
    fetch_enabled();

    async function change_value() {
        let value = reg.enabledValue;
        if (enabled) {
            value = reg.standardValue;
        }

        await invoke("change_dword", { hkey: reg.hkey, path: reg.path, key: reg.key, value: value });
    }
</script>

<main>
    <div id="content">
        <div id="wrapper">
            <div id="title">
                <div id="arrow">
                    <div id="arrow-container" on:click={toggle_extend}>
                        {#if extended}
                            <Icon name={"expand_more"} size={32}/>
                        {:else}
                            <Icon name={"expand_less"} size={32}/>
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
                            <input type="checkbox" id="switch" bind:checked={enabled} on:click={change_value}>
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
        margin-left: 41px;
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