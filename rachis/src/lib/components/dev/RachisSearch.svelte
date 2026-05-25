<script lang="ts">
    import type { Rachis } from "$lib/types/Rachis";
    import { sessionStore as ss } from "$lib/stores/session.svelte";

    let search_str: string = $state<string>("");
    let arg: string | null = $derived(search_str == "" ? null : search_str);
</script>

<main>
    <div>
        <pre>Search Rachis (leave empty to get all Rachises):</pre>
        <input type="text" bind:value={search_str} />
        <button onclick={(): Promise<Rachis[]> => ss.getRachises(arg)}
            >Search</button>
    </div>
    <div>
        <pre>Searching for: {search_str == "" ? "All Rachises" : (
                search_str
            )}</pre>
        <pre>Rachises found: {JSON.stringify(ss.rachises_found, null, 4)}</pre>
    </div>
</main>
