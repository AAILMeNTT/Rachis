<script lang="ts">
    import Star from "@lucide/svelte/icons/star";
    import Settings from "@lucide/svelte/icons/settings";
    import Trash2 from "@lucide/svelte/icons/trash";
    import { goto } from "$app/navigation";
    import { landingStore } from "$lib/stores/landing.svelte";
    import { dialogs } from "$lib/stores/dialog.svelte";
    import type { RegistryEntry } from "$lib/types/RegistryEntry";

    interface Props {
        /** The Flight to display */
        flight: RegistryEntry;
    }

    let { flight }: Props = $props();

    // ——— Derived Values ———

    /** Whether this Flight was the most recently opened one */
    let isLastActive: boolean = $derived(
        landingStore.mostRecent?.id === flight.id
    );

    /** "Time since last edited" string */
    let timeSinceEdited: string = $derived.by((): string => {
        const now: number = Date.now();
        const then: number = new Date(flight.last_opened_at).getTime();
        const diffMs: number = now - then;

        const minutes: number = Math.floor(diffMs / 60000);
        if (minutes < 1) return "just now";
        if (minutes < 60) return `${minutes} minutes ago`;

        const hours: number = Math.floor(minutes / 60);
        if (hours < 24) return `${hours} hour(s) ago`;

        const days: number = Math.floor(hours / 24);
        if (days < 30) return `${days} day(s) ago`;

        const months: number = Math.floor(days / 30);
        if (months < 12) return `${months} month(s) ago`;

        return `a long time ago`;
    });

    /** Formatted word count (e.g., "87,209 words") */
    let wordCountDisplay: string = $derived(
        `${flight.word_count.toLocaleString()} words`
    );

    /** Navigates to the Flight's workspace when clicked */
    function handleClick(): void {
        goto(`/workspace/${flight.id}`);
    }

    /** Toggle the favorite status of the Flight */
    async function handleToggleFavorite(event: MouseEvent): Promise<void> {
        // Stop the click from being handled by the card's click handler
        event.stopPropagation();
        await landingStore.toggleFavorite(flight.id);
    }

    /** Removes the Flight from the landing store */
    async function handleRemove(event: MouseEvent): Promise<void> {
        // Stop the click from being handled by the card's click handler
        event.stopPropagation();

        // Pop up a ConfirmDialog to verify the user's action
        const confirmed: boolean = await dialogs.confirm(
            "Delete Flight",
            `Are you sure you want to delete "${flight.name}"? This cannot be undone.`,
            { confirmText: "Delete", severity: "destructive" }
        );

        // If confirmed, remove the Flight and display a NotifyDialog
        if (confirmed) {
            await landingStore.remove(flight.id);
            dialogs.notify("Flight deleted", { severity: "success" });
        }
    }

    /** Opens the Flight settings dialog */
    function handleSettings(event: MouseEvent): void {
        // Stop the click from being handled by the card's click handler
        event.stopPropagation();
        // TODO: actually open flight settings dialog
        console.log("Settings for:", flight.id);
    }
</script>

<div
    class="flight-card
    bg-white w-full h-auto rounded-2xl flex flex-row flex-nowrap
    items-center content-stretch justify-between px-8 py-4 cursor-pointer"
    onclick={handleClick}
    role="button"
    tabindex="0"
    onkeydown={(e: KeyboardEvent) => e.key === "Enter" && handleClick()}>
    <!-- //— Left content: status dot + flight info —// -->
    <div
        class="left-content
        w-full h-full flex flex-row flex-nowrap items-center
        content-stretch justify-start gap-x-2">
        <div
            class="circle
            w-2.5 h-2.5 rounded-full {isLastActive ? 'bg-lilac' : (
                'bg-cornsilk outline-2 outline-wisteria outline-solid'
            )}"
            title={isLastActive ? "Most recently opened" : undefined}>
        </div>
        <div
            class="flight-info
            w-full h-full flex flex-col flex-nowrap items-start
            content-stretch justify-center gap-y-2.25">
            <p class="flight-title">{flight.name}</p>
            <div class="info">
                <p>{wordCountDisplay} · {timeSinceEdited}</p>
            </div>
        </div>
    </div>

    <!-- //— Right content: action buttons —// -->
    <div
        class="right-content
        w-auto h-auto flex flex-row flex-nowrap items-center
        content-stretch justify-between gap-x-4"
        role="group"
        aria-label="Flight actions">
        <button
            onclick={handleToggleFavorite}
            aria-label={flight.is_favorite ?
                "Remove from favorites"
            :   "Add to favorites"}
            title={flight.is_favorite ?
                "Remove from favorites"
            :   "Add to favorites"}>
            <Star
                color={flight.is_favorite ? "var(--color-lilac)" : undefined}
                fill={flight.is_favorite ? "var(--color-lilac)" : (
                    "#00000000"
                )} />
        </button>
        <!-- TODO: would be nice if these buttons like pulsed a little on hover perhaps -->
        <button
            onclick={handleSettings}
            aria-label="Flight settings"
            title="Flight settings">
            <Settings />
        </button>
        <button
            onclick={handleRemove}
            aria-label="Delete flight"
            title="Delete flight">
            <Trash2 />
        </button>
    </div>
</div>

<style>
    .flight-card:focus-visible {
        outline: 2px solid var(--color-lilac);
        outline-offset: 2px;
    }

    .right-content button {
        cursor: pointer;
        background: none;
        border: none;
        padding: 4px;
        border-radius: 5px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .right-content button:hover {
        background: rgba(214, 175, 232, 0.12);
    }
</style>
