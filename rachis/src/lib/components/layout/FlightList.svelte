<script lang="ts">
    import { type RegistryEntry } from "$lib/types/RegistryEntry";
    import FlightCard from "$lib/components/layout/FlightCard.svelte";
    import { landingStore } from "$lib/stores/landing.svelte";

    interface Props {
        flights: RegistryEntry[];
    }

    let { flights }: Props = $props();
</script>

<div
    class="flight-list
    w-full h-auto flex flex-col flex-nowrap items-start content-stretch justify-start gap-y-4 pe-12 py-6">
    {#each flights as flight (flight.id)}
        {#if flight.id === landingStore.mostRecent?.id}
            <FlightCard {flight} isFeatured={true} />
        {:else}
            <FlightCard {flight} />
        {/if}
    {/each}
</div>
