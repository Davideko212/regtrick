<script>
	import RegtrickEntry from '../components/RegtrickEntry.svelte';
	import Navbar from '../components/Navbar.svelte';
	import { activePanel, ActivePanel, explorerWidth } from '../stores.js';
	import Entries from "../../public/assets/entries.json";
	import Explorer from '../components/Explorer.svelte';

	let width = 300;

	explorerWidth.subscribe(value => {
		width = value;
	})
</script>

<main>
	<Navbar/>
	<Explorer/>

	<div id="content" style="--explorer-padding: {width+30}px">
		{#each Entries as entry}
			<RegtrickEntry {...entry}/>
		{/each}
	</div>
</main>

<style>
	* {
        margin: 0;
        padding: 0;
		border: 0;
    }

	#content {
		padding-left: var(--explorer-padding);
        padding-right: 100px;
	}
</style>