import { type Flight } from "$lib/types/Flight";
import { type Tree } from "$lib/types/Tree";
import { invoke } from "@tauri-apps/api/core";
import { trace } from "@tauri-apps/plugin-log";

class WorkspaceStore {
    /** Whether the workspace is currently loading */
    loading: boolean = $state(false);

    /** The current Flight */
    current_flight: Flight | null = $state<Flight | null>(null);

    /** The Tree state */
    tree: Tree | null = $state<Tree | null>(null);

    /**
     * Load the workspace tree for a given Flight ID.
     *
     * @param id The ID of the Flight to load the tree for.
     */
    async loadTree(id: string): Promise<void> {
        this.loading = true;
        trace(`WorkspaceStore: loading tree for Flight with ID: ${id}...`);
        this.tree = await invoke<Tree>("load_layout", { id });
        trace(`WorkspaceStore: successfully loaded tree.`);
        this.loading = false;
    }

    /**
     * Retrieves a Flight by its ID and loads the workspace tree from the FlightContext.
     *
     * @param flight_id The ID of the Flight to retrieve.
     *
     * @returns The Flight object if found, otherwise null.
     */
    async getFlightById(flight_id: string): Promise<Flight | null> {
        this.current_flight = await invoke<Flight | null>("get_flight", {
            flight_id,
        });
        if (this.current_flight) this.loadTree(flight_id);
        return this.current_flight;
    }
}

export const workspaceStore = new WorkspaceStore();
