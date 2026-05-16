<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let input = $state("");
    let result: object | null = $state(null);
    let error = $state("");

    async function parseTag() {
        try {
            const match = input.match(/^[celin]![^!]+!$/);
            if (match) {
                result = await invoke("parse_tag", { input });
            }
            error = "";
        } catch (e) {
            error = String(e);
            result = null;
        }
    }

    async function getFlight() {
        try {
            result = await invoke("get_flight");
            error = "";
        } catch (e) {
            error = String(e);
            result = null;
        }
    }

    async function insertFlight() {
        try {
            if (input) {
                result = await invoke("create_flight", { name: input });
            }
            error = "";
        } catch (e) {
            error = String(e);
            result = null;
        }
    }
</script>

<main class="container">
    <!-- Tag Parser
    <input
        bind:value={input}
        oninput={parseTag}
        placeholder="Type a tag (e.g. c!Twilight Sparkle!)"
    /> -->
    <!-- Flight Insertion -->
    <input
        bind:value={input}
        oninput={insertFlight}
        placeholder={`Type the name of your Flight (e.g. "My Little Pony: Ad Eternum")`}
    />
    <!-- Flight Query -->
    <button onclick={getFlight}> Get Flights </button>
    {#if result !== null}
        <pre>{JSON.stringify(result, null, 4)}</pre>
    {/if}

    {#if error}
        <p style="color: red">{error}</p>
    {/if}
</main>
