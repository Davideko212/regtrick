<script>
    export let name;
    export let size = 24;
</script>

<main style="--size: {size}px">
    <!-- Material Icons -->
    <link rel="stylesheet" href="https://fonts.googleapis.com/icon?family=Material+Icons" />
    
    <h1><span class="material-icons">{name}</span></h1>
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