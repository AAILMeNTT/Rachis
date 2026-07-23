<script lang="ts">
    import LandingSidebar from "$lib/components/layout/LandingSidebar.svelte";
    import LandingContent from "$lib/components/layout/LandingContent.svelte";
    
    interface Props {}

    let {}: Props = $props();
</script>

<div
    class="landing
    w-full h-full flex flex-row flex-nowrap items-start content-stretch justify-start">
    <LandingSidebar />
    <LandingContent />
</div>
