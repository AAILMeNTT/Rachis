<script lang="ts">
    import EditorWidget from "$lib/components/widgets/EditorWidget.svelte";
    import RachisSearch from "$lib/components/dev/RachisSearch.svelte"
    
    // TODO: It would be really nice to have a, say, <RachisLoader /> component that I could use
    // to, like, input a UUID, then it would assign the corresponding Rachis to a given variable.
    // Any way to do that?
</script>

<main>
    <a href="../..">To Home</a>

    <h1>Editor Widget</h1>
    
    <RachisSearch />
    <EditorWidget />
</main>