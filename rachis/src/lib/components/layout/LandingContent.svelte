<script lang="ts">
    import "$lib/styles/app.css";
    import Section from "$lib/components/layout/Section.svelte";
    import { landingStore } from "$lib/stores/landing.svelte";

    interface Props {}

    let {}: Props = $props();
</script>

<div
    class="landingContent
    h-full grid auto-rows-auto">
    {#if landingStore.mostRecent}
        <div
            class="welcome-back
                col-span-full">
            <Section
                title="Welcome Back"
                subtitle="Your latest writing, ready for flight"
                flights={[landingStore.mostRecent]} />
        </div>
    {/if}
    <div
        class="favorites
        col-span-3fr">
        <Section
            title="Favorites"
            subtitle="Your favorite eggs, all in one basket"
            flights={landingStore.favorites} />
    </div>
    <div
        class="older-projects
        col-span-2fr">
        <Section
            title="Older Projects"
            subtitle="Your previous works, comfortably settled in"
            flights={landingStore.nonfavorites} />
    </div>
</div>

<style>
    /* TODO: at some point convert all this to TailwindCSS
       Leave these commented as they're converted; remove
       this section ONLY after everything has been converted */
    .landingContent {
        /*grid-template-rows: auto auto;*/
        grid-template-columns: 3fr 2fr;
        align-items: start;
        align-content: start;
        justify-content: stretch;
        row-gap: 72px;
        column-gap: 32px;
        padding-inline: 100px 320px;
        padding-block-start: 60px;
    }
</style>
