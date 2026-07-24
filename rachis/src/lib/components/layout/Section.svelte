<script lang="ts">
    import { type RegistryEntry } from "$lib/types/RegistryEntry";
    import SectionHeader from "$lib/components/layout/SectionHeader.svelte";
    import FlightList from "$lib/components/layout/FlightList.svelte";
    import FlightCard from "$lib/components/layout/FlightCard.svelte";

    interface Props {
        title: string;
        subtitle?: string;
        flights?: RegistryEntry[];
    }

    let { title, subtitle, flights }: Props = $props();
</script>

<div
    class="welcome-back
    w-full h-auto flex flex-col flex-nowrap items-center content-stretch justify-start gap-y-4">
    <SectionHeader {title} {subtitle} />
    {#if flights}
        <FlightList {flights} />
    {/if}
    <!-- TODO: The `title` and `isFavourite` values for the FlightCard should receive data about the user's last-used Rachis -->
</div>
