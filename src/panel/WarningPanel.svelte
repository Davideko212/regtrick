<script>
    import { activePanel, ActivePanel } from '../stores.js';
    
	function acknowledge() {
        activePanel.set(ActivePanel.Main);
    }

    let yes = false;
</script>

<main>
    <div id="content">
        <h1>WARNING</h1>
        
        <p>
            This piece of software directly manipulates values present in the Windows Registry of this machine. The creator of this software takes no responsibility for any kind
            of damage done to the Windows OS on this machine by using it.
        </p>
        
        <div id="check-container">
            <input type="checkbox" id="checkbox" bind:checked={yes}>
            <label for="checkbox">By using this software you acknowledge the fact that it may make your Windows unusable.</label>
        </div>
    
        <button on:click={acknowledge} disabled={!yes}>
            Go to the Main panel
        </button>
    </div>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
		border: 0;
    }

    main {
        display: flex;
        height: 100%;
    }

    #content {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 50%;
        margin: auto;
        gap: 20px;
    }

    #check-container {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 10px;
    }

    #checkbox {
        position: relative;
        vertical-align: middle;
    }

    input {
        width: 30px;
        height: 30px;
        cursor: pointer;
    }

    button {
        margin: 30px;
        cursor: pointer;
    }

    h1 {
        font-size: 3em;
        color: red;
    }
</style>