<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Flight } from "$lib/types/Flight";

    let flightName: string = $state("");
    let flight: Flight | null = $state(null);
    let errorMessage: string = $state("");

    async function createFlight() {
        errorMessage = "";
        if (!flightName.trim()) {
            errorMessage = "Flight name cannot be empty";
            return;
        }
        try {
            const result = await invoke<Flight>("create_flight", {
                name: flightName,
            });
            flight = result;
            flightName = "";
        } catch (error) {
            errorMessage = String(error);
            console.error(error);
        }
    }

    async function getFlight() {
        errorMessage = "";
        try {
            const result = await invoke<Flight | null>("get_flight");
            flight = result;
        } catch (error) {
            errorMessage = String(error);
            console.error(error);
        }
    }

    async function deleteFlight() {
        errorMessage = "";
        if (!flight) {
            errorMessage = "No flight loaded to delete";
            return;
        }
        try {
            await invoke("delete_flight", { id: flight.id });
            flight = null;
        } catch (error) {
            errorMessage = String(error);
            console.error(error);
        }
    }
</script>

<div>
    <div>
        <p>Flight name:</p>
        <input type="text" bind:value={flightName} />
    </div>
    <div>
        <button onclick={createFlight}>Create New Flight</button>
        <button onclick={getFlight}>Get Flight</button>
        <button onclick={() => (flightName = "")}>Reset</button>
        <button onclick={deleteFlight}>Delete Flight</button>
    </div>
    <div>
        <pre>{JSON.stringify(flight, null, 4)}</pre>
    </div>
    {#if errorMessage}
        <p style="color: red">{errorMessage}</p>
    {/if}
</div>
