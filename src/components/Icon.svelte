<script>
    import Icons from "../../public/assets/icons/icons.json";

    export let name;
    export let size = 24;
    let own = false;

    for (const key in Icons) {
        if (key == name) {
            own = true;
        }
    }
</script>

<main style="--size: {size}px">
    <!-- Material Icons -->
    <link rel="stylesheet" href="https://fonts.googleapis.com/icon?family=Material+Icons" />
    
    {#if own}
        <img src="{"assets/icons/" + Icons[name].path}" alt="{Icons[name].alt}">
    {:else}
        <h1><span class="material-icons">{name}</span></h1>
    {/if}
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        border: 0;
    }

    main {
        background-color: transparent;
    }

    img {
        height: var(--size);
        width: var(--size);
        aspect-ratio: 1/1;

        display: flex;
        align-items: center;
    }

    h1 {
        height: var(--size);
        width: var(--size);
        aspect-ratio: 1/1;

        display: flex;
        align-items: center;
    }

    h1 .material-icons {
        font-size: var(--size);
    }
</style>