import { type Rachis } from "$lib/types/Rachis";
import { invoke } from "@tauri-apps/api/core";

// Create a class for the store
class SessionStore {
    // State for the rachises found
    rachises_found: Rachis[] = $state<Rachis[]>([]);

    // State for currently-loaded Rachis
    current_rachis: Rachis | null = $state<Rachis | null>(null);

    // A function to update rachises_found
    async getRachises(title?: string | null): Promise<Rachis[]> {
        let result: Rachis[];
        if (title) {
            result = await invoke<Rachis[]>("get_rachises_by_title", {
                title,
            });
        } else {
            result = await invoke<Rachis[]>("list_rachises");
        }
        this.rachises_found = result;
        return result;
    }

    // A function to update current_rachis
    async loadRachis(id: string): Promise<Rachis | null> {
        const result: Rachis | null = await invoke<Rachis | null>(
            "get_rachis_by_id",
            {
                id,
            }
        );
        this.current_rachis = result;
        return result;
    }

    async unloadRachis(): Promise<void> {
        this.current_rachis = null;
    }
}

export const sessionStore = new SessionStore();
