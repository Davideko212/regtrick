<script>
    import { activePanel, ActivePanel, activeTheme, ActiveTheme } from '../stores.js';
    import Icon from '../components/Icon.svelte';
    import Fab from '@smui/fab';

    let theme;
    let active;

    function switchPanel(panel) {
        if (active != panel) {
            activePanel.set(panel);
        }
    }

    activePanel.subscribe(value => {
        active = value;
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
    <div id="window">
        <Fab class="clickable" on:click={() => switchPanel(ActivePanel.Main)}>
            <Icon name={"Logo_greyscale"} id="icon"/>
        </Fab>
        <Fab class="clickable" on:click={() => switchPanel(ActivePanel.Options)}>
            <Icon name={"settings"} id="icon"/>
        </Fab>
        <Fab class="clickable" on:click={() => switchPanel(ActivePanel.Info)}>
            <Icon name={"info"} id="icon"/>
        </Fab>
    </div>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        border: 0;
    }

    #window {
        display: flex;
        flex-direction: column;
        gap: 15px;
        position: fixed;
        top: 0%;
        right: 0%;
        margin-top: -15px;
        margin-right: -15px;
        padding: 10px;
        padding-top: 25px;
        padding-right: 25px;

        border-radius: 15px;
    }

    #window .clickable {
        padding: 10px;
        border-radius: 50%;
        cursor: pointer;

        background-color: pink;
    }
</style>