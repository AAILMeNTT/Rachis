<script lang="ts">
    import "$lib/styles/app.css";
    import Stats from "$lib/components/layout/Stats.svelte";

    interface Props {
        title: string;
        subtitle?: string;
    }
    let { title, subtitle }: Props = $props();
</script>

<div
    class="header
    bg-white w-full h-auto flex flex-row flex-nowrap content-stretch items-center justify-between px-10 py-6 rounded-2xl">
    <div
        class="header-welcome
        w-auto h-auto flex flex-col flex-nowrap content-stretch items-start justify-start gap-y-2 py-3">
        <p>{title}</p>
        {#if subtitle}
            <p>{subtitle}</p>
        {/if}
    </div>
    <div
        class="header-stats
        w-auto h-full flex flex-row flex-nowrap content-stretch items-center justify-between gap-x-8">
        <Stats />
    </div>
</div>
