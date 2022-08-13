<script>
    import { explorerWidth } from '../stores.js';
    import Icon from './Icon.svelte';

    let width = 300;
    
    explorerWidth.subscribe(value => {
		width = value;
	})
</script>

<main>
    <div id="content" style="--width: {width}px">
        
    </div>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        border: 0;
    }
</style>