<script>
    import { Icons } from '../stores.js';

    export let name;

    let path = "../" + Icons[name].path;
    let alt = Icons[name].alt;
    let width = Icons[name].width;
    let height = Icons[name].height;
</script>

<main>
    <img src="{path}" alt="{alt}" width="{width}" height="{height}">
</main>

<style>

</style>