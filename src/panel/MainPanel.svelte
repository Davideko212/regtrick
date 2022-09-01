<script>
	import RegtrickEntry from '../components/RegtrickEntry.svelte';
	import Navbar from '../components/Navbar.svelte';
	import { explorerWidth, selectedEntries, activeTheme, ActiveTheme } from '../stores.js';
	import Entries from "../../src-tauri/src/entries.json";
	import Explorer from '../components/Explorer.svelte';

	let theme;
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

	activeTheme.subscribe(value => {
		theme = value;
	})
</script>

<svelte:head>
	<!-- SMUI Styles -->
	{#if theme === ActiveTheme.Undefined}
		<link rel="stylesheet" href="build/smui.css" media="(prefers-color-scheme: light)" />
		<link rel="stylesheet" href="build/smui-dark.css" media="screen and (prefers-color-scheme: dark)" />
	{:else if theme === ActiveTheme.Dark}
		<link rel="stylesheet" href="build/smui.css" />
		<link rel="stylesheet" href="build/smui-dark.css" media="screen" />
	{:else if theme === ActiveTheme.FullDark}
		 <!-- WIP -->
	{:else}
		<link rel="stylesheet" href="build/smui.css" />
	{/if}
</svelte:head>

<main>
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