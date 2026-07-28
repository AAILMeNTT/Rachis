<script lang="ts">
    import { onMount } from "svelte";
    import type { PageProps } from "./$types";
    import { workspaceStore } from "$lib/stores/workspace.svelte";
    import Workspace from "$lib/components/workspace/Workspace.svelte";

    let loading: boolean = $derived<boolean>(workspaceStore.loading);
    let error: Error | null = $state<Error | null>(null);
    let { params, data }: PageProps = $props();

    onMount(async (): Promise<void> => {
        loading = true;
        try {
            await workspaceStore.getFlightById(params.id);
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
    {#if workspaceStore.tree}
        <Workspace root={workspaceStore.tree.root} />
    {:else}
        <p>No workspace available!</p>
    {/if}
{/if}