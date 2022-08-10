<script>
    import Icon from '../components/Icon.svelte';

    let on = false;
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
                <h1>Systemclock show seconds</h1>
            </div>
            {#if extended}
                <div id="entry">
                    <input type="checkbox" id="switch" bind:checked={on}>
                    <p>System clock goes brrrrr</p>
                </div>
            {/if}
        </div>
    </div>
</main>

<style>
    #content {
        display: flex;
        flex-direction: row;
        border: 5px;
        padding-left: 10%;
        padding-right: 10%;
    }

    #entry {
        display: flex;
        flex-direction: row;
    }

    h1 {
        margin: 0;
    }

    #title {
        margin-top: 3%;
        margin-bottom: 3%;
        display: flex;
        flex-direction: row;
        gap: 8px;
    }

    #arrow {
        margin-top: 4%;
    }
</style>