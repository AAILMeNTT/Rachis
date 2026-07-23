<script lang="ts">
    import { onMount } from "svelte";
    import type { PageProps } from "./$types";
    import { workspaceStore } from "$lib/stores/workspace.svelte";
    // import Workspace from "$lib/components/workspace/Workspace.svelte";

    let loading: boolean = $state<boolean>(true);
    let error: Error | null = $state<Error | null>(null);
    let { params, data }: PageProps = $props();

    onMount(async (): Promise<void> => {
        loading = true;
        try {
            const flight_id: string = params.id;
            workspaceStore.current_flight =
                await workspaceStore.getFlightById(flight_id);
        } catch (e) {
            error = e instanceof Error ? e : new Error(String(e));
        } finally {
            loading = false;
        }
    });
</script>

<p>Params: {JSON.stringify(params, null, 4)}</p>
<p>Data: {JSON.stringify(data, null, 4)}</p>

{#if loading}
    <p>Loading workspace...</p>
{:else}
    <!-- <Workspace /> -->
    <p>Workspace loaded.</p>
{/if}
