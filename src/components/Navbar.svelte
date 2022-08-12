<script>
    import Icon from '../components/Icon.svelte';
</script>

<main>
    <div id="x-container">
        <Icon name={"X"} id="icon"/>
    </div>
    <div id="cog-container">
        <Icon name={"Cog"} id="icon"/>
    </div>
    <div id="info-container">
        <Icon name={"Info"} id="icon"/>
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

        background-color: greenyellow;
        border-radius: 15px;
    }

    #x-container{
       background-color: red;
       padding: 10px;
       border-radius: 50%;
    }

    #cog-container{
       background-color: red;
       padding: 10px;
       border-radius: 50%;
    }

    #info-container{
       background-color: red;
       padding: 10px;
       border-radius: 50%;
    } 
</style>