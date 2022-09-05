<script>
	import RegtrickEntry from '../components/RegtrickEntry.svelte';
	import { explorerWidth, selectedEntries } from '../stores.js';
	import Entries from "../../src-tauri/src/entries.json";
	import Explorer from '../components/Explorer.svelte';

	let width;
	let selected = [];
	$: if (selected.length == 0) {
		Entries.forEach(category => {
			category.entries.forEach(entry => {
				selected.push(entry);
			})
		})
	}

	explorerWidth.subscribe(value => {
		width = value;
	})

	selectedEntries.subscribe(values => {
		selected = values;
	})
</script>

<main>
	<Explorer/>

	<div id="content" style="--explorer-padding: {width+20}px">
		{#key selected}
			{#each selected as entry}
				<RegtrickEntry {...entry}/>
			{/each}
		{/key}
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