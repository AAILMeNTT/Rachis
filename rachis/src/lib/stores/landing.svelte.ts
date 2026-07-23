import { invoke } from "@tauri-apps/api/core";
import { dialogs } from "$lib/stores/dialog.svelte";
import { type FormField } from "$lib/types/Dialog";
import { type RegistryEntry } from "$lib/types/RegistryEntry";
import { type ReconcileReport } from "$lib/types/ReconcileReport";

interface ReconcileReportAcc {
    found: ReconcileReport[];
    moved: ReconcileReport[];
    mismatched: ReconcileReport[];
    discovered: ReconcileReport[];
    total: number;
}

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

    currentFlight: RegistryEntry | undefined = $state<
        RegistryEntry | undefined
    >(undefined);

    // ========================================================================
    // Derived Values
    // ========================================================================

    /**
     * Flights the user has marked as favourites
     */
    favorites: RegistryEntry[] = $derived.by((): RegistryEntry[] => {
        return this.flights.filter(
            (e: RegistryEntry): boolean => e.is_favorite
        );
    });

    /**
     * Flights that aren't favourited
     */
    nonfavorites: RegistryEntry[] = $derived.by((): RegistryEntry[] => {
        return this.flights.filter(
            (e: RegistryEntry): boolean => !e.is_favorite
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
            .map((e: RegistryEntry): number => e.word_count)
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
     * Sets the current flight by ID, returning the entry if found.
     *
     * @param id The ID of the flight to set as current.
     *
     * @returns The `RegistryEntry` if found, otherwise `undefined`.
     */
    setCurrent(id: string): RegistryEntry | undefined {
        this.currentFlight = this.flights.find((f) => f.id === id);
        return this.currentFlight;
    }

    // TODO: There should probably be a button in the Settings that can proc this manually, yeah?
    async reconcileFlights(): Promise<void> {
        let reports: ReconcileReportAcc = await invoke<ReconcileReport[]>(
            "reconcile_registry_flights"
        ).then((r: ReconcileReport[]): ReconcileReportAcc =>
            r.reduce(
                (
                    acc: ReconcileReportAcc,
                    r: ReconcileReport
                ): ReconcileReportAcc => {
                    switch (r.status.type) {
                        case "Found":
                            acc.found.push(r);
                            break;
                        case "Moved":
                            acc.moved.push(r);
                            break;
                        case "Mismatch":
                            acc.mismatched.push(r);
                            break;
                        case "Discovered":
                            acc.discovered.push(r);
                            break;
                    }
                    acc.total++;
                    return acc;
                },
                {
                    found: [],
                    moved: [],
                    mismatched: [],
                    discovered: [],
                    total: 0,
                }
            )
        );

        // Create a Notify dialog if there are no Moved, Mismatched, or Discovered reports
        if (reports.total === reports.found.length && reports.total > 0) {
            dialogs.notify(`All ${reports.total} Flight(s) accounted for!`);
        }
        // Otherwise, create an Alert dialog that shows the user the moved, mismatched, and discovered Flights
        else {
            const moved: FormField[] = reports.moved.map(
                (r: ReconcileReport): FormField => {
                    // lowkey i hate typescript but i can't fault it for my inadequacies
                    if (r.status.type !== "Moved")
                        throw new Error("unreachable");
                    return {
                        id: r.id,
                        label: `${r.name} (moved from ${r.status.data.old_path} to ${r.status.data.new_path})`,
                        type: "info",
                    };
                }
            );
            const mismatched: FormField[] = reports.mismatched.map(
                (r: ReconcileReport): FormField => ({
                    id: r.id,
                    label: r.name,
                    type: "radio",
                    layout: "horizontal",
                    options: [
                        { value: "unregister", label: "Unregister" },
                        { value: "keep", label: "Keep" },
                    ],
                    required: true,
                })
            );
            const discovered: FormField[] = reports.discovered.map(
                (r: ReconcileReport): FormField => ({
                    id: r.id,
                    label: r.name,
                    type: "radio",
                    layout: "horizontal",
                    options: [
                        { value: "register", label: "Register" },
                        { value: "ignore", label: "Ignore" },
                    ],
                    required: true,
                })
            );
            const fields: FormField[] = moved
                .concat(mismatched)
                .concat(discovered);

            const result: Record<string, string> | null = await dialogs.form(
                `Rachis found some discrepancies!`,
                fields,
                { submitText: "Done" }
            );

            if (result) {
                // Collect all the removals and registrations
                const toRemove: string[] = [];
                const toRegister: {
                    name: string;
                    path: string;
                    id: string;
                }[] = [];

                for (const id of Object.keys(result)) {
                    switch (result[id]) {
                        case "unregister":
                            toRemove.push(id);
                            break;
                        case "register": {
                            // Find the report to get the path and name
                            const r: ReconcileReport | undefined =
                                reports.discovered.find(
                                    (r: ReconcileReport): boolean => r.id === id
                                );
                            if (r && r.status.type === "Discovered") {
                                toRegister.push({
                                    name: r.name,
                                    path: r.status.data.path,
                                    id: r.id,
                                });
                            }
                            break;
                        }
                        // "keep" and "ignore" are ignored for now
                    }
                }

                // Execute removals
                for (const id of toRemove) {
                    try {
                        await invoke<boolean>("remove_registry_flight", { id });
                    } catch (e) {
                        console.error(`Failed to remove flight ${id}:`, e);
                    }
                }

                // Execute registrations
                for (const flight of toRegister) {
                    try {
                        await invoke<RegistryEntry>("add_registry_flight", {
                            flight_name: flight.name,
                            flight_path: flight.path,
                            flight_id: flight.id,
                        });
                    } catch (e) {
                        console.error(
                            `Failed to register flight ${flight.id}:`,
                            e
                        );
                    }
                }

                // Reload to reflect changes
                await this.loadAll();
            }
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
     * Looks up the current status from the loaded flights list.
     *
     * @param id The UUID of the Flight to toggle.
     *
     * @returns The new favourite status (true = now favourited).
     */
    async toggleFavorite(id: string): Promise<boolean> {
        this.is_loading = true;
        this.error = null;

        try {
            // Look up current status to compute the toggle
            const current = this.setCurrent(id)?.is_favorite ?? false;

            const entry: RegistryEntry = await invoke<RegistryEntry>(
                "update_registry_flight",
                { id, patch: { is_favorite: !current } }
            );
            await this.loadAll();
            return entry.is_favorite;
        } catch (e) {
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
    async add(name: string, path: string, id: string): Promise<RegistryEntry> {
        this.is_loading = true;
        this.error = null;

        try {
            const result: RegistryEntry = await invoke<RegistryEntry>(
                "add_registry_flight",
                { flight_name: name, flight_path: path, flight_id: id }
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
