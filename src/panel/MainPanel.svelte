<script>
	import RegtrickEntry from '../components/RegtrickEntry.svelte';
	import Navbar from '../components/Navbar.svelte';
	import Icon from '../components/Icon.svelte';
	import { activePanel, ActivePanel } from '../stores.js';
    
</script>

<main>
	<Navbar/>

	<div id="content">
		<RegtrickEntry/>
		<RegtrickEntry/>
		<RegtrickEntry/>
	</div>
</main>

<style>
	* {
        margin: 0;
        padding: 0;
    }
</style>