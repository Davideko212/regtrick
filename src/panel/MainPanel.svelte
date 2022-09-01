<script>
	import RegtrickEntry from '../components/RegtrickEntry.svelte';
	import Navbar from '../components/Navbar.svelte';
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
	<!-- SMUI Styles -->
    <link rel="stylesheet" href="build/smui.css" media="(prefers-color-scheme: light)" />
    <link rel="stylesheet" href="build/smui-dark.css" media="screen and (prefers-color-scheme: dark)" />

	<Navbar/>
	<Explorer/>

	<div id="content" style="--explorer-padding: {width+30}px">
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