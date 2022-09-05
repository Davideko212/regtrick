<script>
    import Select, { Option } from '@smui/select';
    import { activeTheme, ActiveTheme } from '../stores.js';

    let currentTheme;
    $: activeTheme.set(currentTheme);

    activeTheme.subscribe(value => {
        currentTheme = value;
    })
</script>

<main>
    <div id="content">
        <h1>Options</h1>
        <div>
            <Select bind:value={currentTheme} label="Select theme">
                {#each ActiveTheme as theme, i}
                    <Option value={i}>{theme.name}</Option>
                {/each}
            </Select>
        </div>
        <p>{currentTheme}</p>
    </div>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
		border: 0;
    }
</style>