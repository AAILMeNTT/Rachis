import { type Rachis } from "$lib/types/Rachis";
import { invoke } from "@tauri-apps/api/core";

class SessionStore {
    /**
     * The Rachises found based on the current search title
     */
    rachises_found: Rachis[] = $state<Rachis[]>([]);

    /**
     * The currently-loaded Rachis
     */
    current_rachis: Rachis | null = $state<Rachis | null>(null);

    /**
     * Retrieves Rachises based on the provided title
     *
     * @param title The title to match Rachises against
     *
     * @returns The retrieved Rachises
     */
    async getRachisesByTitle(title?: string | null): Promise<Rachis[]> {
        return await invoke<Rachis[]>("get_rachises_by_title", {
            title,
        });
    }

    /**
     * Retrieves a Rachis by its ID
     *
     * @param id The ID of the Rachis to retrieve
     * 
     * @returns The retrieved Rachis
     */
    async getRachisById(id: string): Promise<Rachis | null> {
        return await invoke<Rachis | null>("get_rachis_by_id", {
            id,
        });
    }

    /**
     * Sets the current Rachis by ID.
     * 
     * If the ID is provided, fetches the Rachis by ID; otherwise, sets to null
     *
     * @param id The ID of the Rachis to set as current
     *
     * @returns A Promise that resolves when the Rachis has been set
     */
    async setRachisById(id?: string): Promise<void> {
        // If the ID is provided, fetch the Rachis by ID; otherwise, set to null
        this.current_rachis =
            id != null ?
                await invoke<Rachis | null>("get_rachis_by_id", {
                    id,
                })
            :   null;
    }
}

export const sessionStore = new SessionStore();
