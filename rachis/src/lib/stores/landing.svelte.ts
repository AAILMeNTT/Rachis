import { type RegistryEntry } from "$lib/types/RegistryEntry";
import { invoke } from "@tauri-apps/api/core";

/**
 * Manages the landing page data/the Flight registry.
 *
 * Handles:
 * - Loading the list of known Flights
 * - Searching, favouriting, adding, and removing Flights
 * - Deriving computed values (favorites, most recent, totals)
 * - More stuff later probably
 */
class LandingStore {
    /**
     * All known Flights, loaded from the registry.
     * THE source of truth; any changes to the registry file MUST update this value or else nothing else will reflect those changes.
     */
    flights: RegistryEntry[] = $state<RegistryEntry[]>([]);

    // ========================================================================
    // Derived Values
    // ========================================================================

    /**
     * Flights the user has marked as favourites
     */
    favorites: RegistryEntry[] = $derived.by((): RegistryEntry[] => {
        return this.flights.filter(
            (flight: RegistryEntry): boolean => flight.is_favorite
        );
    });

    /**
     * Flights that aren't favourited
     */
    nonfavorites: RegistryEntry[] = $derived.by((): RegistryEntry[] => {
        return this.flights.filter(
            (flight: RegistryEntry): boolean => !flight.is_favorite
        );
    });

    /**
     * The most recently opened Flight, or undefined if none exist
     */
    mostRecent: RegistryEntry | undefined = $derived.by(
        (): RegistryEntry | undefined => {
            if (this.flights.length === 0) return undefined;

            return [...this.flights].sort(
                (a: RegistryEntry, b: RegistryEntry): number => {
                    return (
                        new Date(b.last_opened_at).getTime()
                        - new Date(a.last_opened_at).getTime()
                    );
                }
            )[0];
        }
    );

    /**
     * Total number of registered Flights
     */
    total_flights: number = $derived(this.flights.length);

    /**
     * Sum of all word counts across every Flight
     */
    total_word_count: number = $derived.by((): number => {
        return this.flights
            .map((flight: RegistryEntry): number => flight.word_count)
            .reduce((a: number, b: number): number => a + b, 0);
    });

    // ========================================================================
    // State
    // ========================================================================

    /**
     * Whether a backend operation is in progress.
     * Components can use this to show loading skeletons.
     */
    is_loading: boolean = $state<boolean>(false);

    /**
     * The most recent error, if any.
     * Components can use this to show error banners or retry prompts.
     * Any new functions or operations must clear upon entry.
     */
    error: Error | null = $state<Error | null>(null);

    /**
     * Current search query for the flights search bar.
     * Not currently wired to anything but soon™️
     */
    search_query: string = $state<string>("");

    // ========================================================================
    // Actions
    // ========================================================================

    /**
     * Loads all Flights from the registry into the store.
     *
     * Call this on landing page mount to populate the dashboard.
     * If it fails, `this.error` is set and the UI can display a retry prompt.
     */
    async loadAll(): Promise<RegistryEntry[]> {
        this.is_loading = true;
        this.error = null;

        try {
            this.flights = await invoke<RegistryEntry[]>(
                "list_registry_flights"
            );
            return this.flights;
        } catch (e: unknown) {
            this.error = e as Error;
            return [];
        } finally {
            this.is_loading = false;
        }
    }

    /**
     * Searches Flights by name (case-insensitive, partial match).
     *
     * @param query The search string to match against Flight names.
     *
     * @returns The filtered list of Flights matching the search query.
     */
    async search(query: string): Promise<RegistryEntry[]> {
        this.is_loading = true;
        this.error = null;

        try {
            this.flights = await invoke<RegistryEntry[]>(
                "search_registry_flights",
                { query }
            );
            return this.flights;
        } catch (e: unknown) {
            this.error = e as Error;
            return [];
        } finally {
            this.is_loading = false;
        }
    }

    /**
     * Toggles the favourite status of a Flight.
     *
     * @param id The UUID of the Flight to toggle.
     *
     * @returns The new favourite status (true = now favourited).
     */
    async toggleFavorite(id: string): Promise<boolean> {
        this.is_loading = true;
        this.error = null;

        try {
            const result: boolean = await invoke<boolean>(
                "toggle_registry_flight_favorite",
                { id }
            );
            await this.loadAll();
            return result;
        } catch (e: unknown) {
            this.error = e as Error;
            return false;
        } finally {
            this.is_loading = false;
        }
    }

    /**
     * Removes a Flight from the registry.
     *
     * @param id The UUID of the Flight to remove.
     *
     * @returns Whether a Flight was actually removed.
     */
    async remove(id: string): Promise<boolean> {
        this.is_loading = true;
        this.error = null;

        try {
            const result: boolean = await invoke<boolean>(
                "remove_registry_flight",
                { id }
            );
            await this.loadAll();
            return result;
        } catch (e: unknown) {
            this.error = e as Error;
            return false;
        } finally {
            this.is_loading = false;
        }
    }

    /**
     * Adds a new Flight to the registry.
     *
     * @param name The human-readable name for the Flight.
     * @param path The absolute path to the `.rachis` database file.
     *
     * @returns The newly created RegistryEntry from the backend.
     */
    async add(name: string, path: string): Promise<RegistryEntry> {
        this.is_loading = true;
        this.error = null;

        try {
            const result: RegistryEntry = await invoke<RegistryEntry>(
                "add_registry_flight",
                { name, path }
            );
            await this.loadAll();
            return result;
        } catch (e: unknown) {
            this.error = e as Error;
            // Return a rejected promise so callers can handle errors too
            throw e;
        } finally {
            this.is_loading = false;
        }
    }
}

export const landingStore = new LandingStore();
