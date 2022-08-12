<script>
    import { Icons } from '../stores.js';

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

<main>
    <img src="{"assets/icons/" + path}" alt="{alt}" width="{width}" height="{height}">
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        border: 0;
    }
</style>