<script lang="ts">
    import { type RegistryEntry } from "$lib/types/RegistryEntry";
    import FlightCard from "$lib/components/layout/FlightCard.svelte";

    interface Props {
      /** The list of Flights to display */
        flights: RegistryEntry[];
    }

    let { flights }: Props = $props();
</script>

<div
    class="flight-list
    w-full h-auto flex flex-col flex-nowrap items-start content-stretch justify-start gap-y-4 pe-12 py-6">
    {#each flights as flight (flight.id)}
        <FlightCard {flight} />
    {/each}
</div>
