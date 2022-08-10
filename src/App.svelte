<script>
	import { activePanel, ActivePanel } from './stores.js';

	import WarningPanel from "./panel/WarningPanel.svelte";
	import MainPanel from "./panel/MainPanel.svelte";

	let activePanelValue;

	//temp
	activePanel.set(ActivePanel.Main);

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