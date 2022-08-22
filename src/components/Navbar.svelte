<script>
    import Icon from '../components/Icon.svelte';

    // TESTING HOW TAURI WORKS
    import { invoke } from "@tauri-apps/api";

    async function bruh() {
        console.log("clicked");
        
        await invoke("test2");
    }
</script>

<main>
    <div id="logo-container" on:click={bruh}>
        <Icon name={"Logo_greyscale"} id="icon"/>
    </div>
    <div id="cog-container">
        <Icon name={"Cog"} id="icon"/>
    </div>
    <div id="info-container">
        <Icon name={"Info"} id="icon"/>
    </div>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        border: 0;
    }

    main {
        display: flex;
        flex-direction: column;
        gap: 15px;
        position: fixed;
        top: 0%;
        right: 0%;
        margin-top: -15px;
        margin-right: -15px;
        padding: 10px;
        padding-top: 25px;
        padding-right: 25px;

        background-color: greenyellow;
        border-radius: 15px;
    }

    #logo-container{
       background-color: red;
       padding: 10px;
       border-radius: 50%;
       cursor: pointer;
    }

    #cog-container{
       background-color: red;
       padding: 10px;
       border-radius: 50%;
       cursor: pointer;
    }

    #info-container{
       background-color: red;
       padding: 10px;
       border-radius: 50%;
       cursor: pointer;
    } 
</style>