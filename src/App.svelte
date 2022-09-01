<script>
	import { activePanel, ActivePanel } from './stores.js';

	import WarningPanel from "./panel/WarningPanel.svelte";
	import MainPanel from "./panel/MainPanel.svelte";
	import OptionsPanel from "./panel/OptionsPanel.svelte";
	import InfoPanel from "./panel/InfoPanel.svelte";

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
{#if activePanelValue === ActivePanel.Options}
	<OptionsPanel />
{/if}
{#if activePanelValue === ActivePanel.Info}
	<InfoPanel />
{/if}

<style>
	
</style>