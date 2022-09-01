<script>
    import Navbar from '../components/Navbar.svelte';
</script>

<main>
    <!-- SMUI Styles -->
    <link rel="stylesheet" href="build/smui.css" media="(prefers-color-scheme: light)" />
    <link rel="stylesheet" href="build/smui-dark.css" media="screen and (prefers-color-scheme: dark)" />

    <Navbar />

    <div id="content">
        <h1>Options</h1>
    </div>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
		border: 0;
    }
</style>