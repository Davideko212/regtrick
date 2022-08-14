<script>
    import Icons from "../../public/assets/icons/icons.json";

    export let name;
    export let width;
    export let height;

    let path = Icons[name].path;
    let alt = Icons[name].alt;

    if (width === undefined) {
        width = Icons[name].width;
    }
    if (height === undefined) {
        height = Icons[name].height;
    }
</script>

<main style="--height: {height}px; --width: {width}px">
    <img src="{"assets/icons/" + path}" alt="{alt}" width="{width}" height="{height}">
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        border: 0;
    }

    main {
        height: var(--height);
        width: var(--width)
    }
</style>