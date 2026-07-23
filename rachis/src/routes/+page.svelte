<script lang="ts">
    import { onMount } from "svelte";
    import { landingStore } from "$lib/stores/landing.svelte";
    import App from "$lib/components/layout/App.svelte";

    onMount(async (): Promise<void> => {
        await landingStore.reconcileFlights();
        await landingStore.loadAll();
    });
</script>

<!-- <main>
    <ul>
        <li>
            {#if import.meta.env.DEV}
                <a href="./dev">To Developer Panel</a>
            {:else}
                <p>To Developer Panel (Disabled)</p>
            {/if}
        </li>
        <li>
            <a href="./dev/widget-test/editor">To Editor Widget Test</a>
        </li>
    </ul>
</main> -->
<App />
