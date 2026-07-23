import { type Flight } from "$lib/types/Flight";
import { type Tree } from "$lib/types/Tree";
import { invoke } from "@tauri-apps/api/core";

class WorkspaceStore {
    /**
     * The current Flight
     */
    current_flight: Flight | null = $state<Flight | null>(null);

    /**
     * The Tree state
     */
    tree: Tree | null = $state<Tree | null>(null);

    async getFlightById(flight_id: string): Promise<Flight | null> {
        // This doesn't exist yet and HOLY HELL did it make me realise i need to rewrite the ENTIRE DATABASE STRUCTURE because im stupid and forgot
        // this.current_flight = await invoke<Flight | null>("get_flight_by_id", { flight_id });
        return this.current_flight;
    }
}

export const workspaceStore = new WorkspaceStore();
