<script>
	import { activePanel, ActivePanel, activeTheme, ActiveTheme } from './stores.js';

	import WarningPanel from "./panel/WarningPanel.svelte";
	import MainPanel from "./panel/MainPanel.svelte";
	import OptionsPanel from "./panel/OptionsPanel.svelte";
	import InfoPanel from "./panel/InfoPanel.svelte";
	import Navbar from './components/Navbar.svelte';

	let activePanelValue;
	let theme;

	activePanel.set(ActivePanel.Warning);

	activePanel.subscribe(value => {
		activePanelValue = value;
	})

	activeTheme.subscribe(value => {
		theme = value;
	})
</script>

<svelte:head>
	<!-- SMUI Styles -->
	{#if theme === 1}
		<link rel="stylesheet" href="build/smui.css" media="screen" />
	{:else if theme === 2}
		<link rel="stylesheet" href="build/smui-dark.css" media="screen" />
	{:else if theme === 3}
		 <!-- WIP -->
	{:else}
		<link rel="stylesheet" href="build/smui.css" />
	{/if}
</svelte:head>

<main>
	{#if !(activePanelValue === ActivePanel.Warning)}
		<Navbar />
	{/if}

	{#if activePanelValue === ActivePanel.Warning}
		<WarningPanel />
	{/if}
	{#if activePanelValue === ActivePanel.Main}
		<MainPanel />
	{/if}
	{#if activePanelValue === ActivePanel.Options}
		<OptionsPanel />
	{/if}
	{#if activePanelValue === ActivePanel.Info}
		<InfoPanel />
	{/if}
</main>

<style>
	* {
        margin: 0;
        padding: 0;
		border: 0;
    }

	main {
		max-width: 100%;
  		overflow-x: hidden;
		height: 100%;
	}

	::-webkit-scrollbar { 
    display: none;
	}
</style>