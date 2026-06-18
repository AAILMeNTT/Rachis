<script lang="ts">
    import "$lib/styles/app.css";
    import { landingStore } from "$lib/stores/landing.svelte";

    interface Props {
        text: string;
        type: "primary" | "secondary" | "tertiary" | "disabled";
        args: string[];
    }

    let { text, type, args }: Props = $props();

    let classNames: string = $derived.by((): string => {
        const bgClass: string =
            type === "primary" ? "bg-lilac"
            : type === "secondary" ? "bg-wisteria"
            : type === "tertiary" ? "bg-cornsilk"
            : "bg-gray-500";

        const cursorClass: string =
            type === "disabled" ? "cursor-not-allowed" : "cursor-pointer";

        return `button w-45 h-9 rounded-xl ${bgClass} ${cursorClass}`;
    });

    async function addRegistryFlight(name: string, path: string) {
        return landingStore.add(name, path);
    }
</script>

{#if type !== "disabled"}
    <!--- TODO: when the button is clicked it should make a dialogue box not just do whatever -->
    <button
        class={classNames}
        onclick={() => addRegistryFlight(args[0], args[1])}>
        {text}
    </button>
{:else}
    <button class={classNames}>
        {text}
    </button>
{/if}
