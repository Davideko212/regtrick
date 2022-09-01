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
</script>

<main>
	<Navbar />

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
	}
</style>