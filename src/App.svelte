<script>
	import { activePanel, ActivePanel } from './stores.js';

	import WarningPanel from "./panel/WarningPanel.svelte";
	import MainPanel from "./panel/MainPanel.svelte";

	let activePanelValue;

	activePanel.set(ActivePanel.Warning);

	activePanel.subscribe(value => {
		activePanelValue = value;
	})
</script>

{#if activePanelValue === ActivePanel.Warning}
	<WarningPanel />
{/if}
{#if activePanelValue === ActivePanel.Main}
	<MainPanel />
{/if}

<style>
	
</style>