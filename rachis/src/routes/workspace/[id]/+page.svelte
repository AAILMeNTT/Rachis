<script lang="ts">
    import { onMount } from "svelte";
    import { landingStore } from "$lib/stores/landing.svelte";

    onMount((): void => {
        landingStore.loadAll();
    });
</script>

<h1>TODO: actually put in the workspace layout for the flight</h1>
