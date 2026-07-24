<script lang="ts">
    import "$lib/styles/app.css";

    interface Props {
        /** The text to display on the button */
        text: string;
        /** The type of button to display */
        type: "primary" | "secondary" | "tertiary" | "disabled";
        /** The function to call when the button is clicked */
        onclick?: () => void;
    }

    let { text, type, onclick }: Props = $props();

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
</script>

{#if type !== "disabled"}
    <button class={classNames} {onclick}>
        {text}
    </button>
{:else}
    <button class={classNames} disabled>
        {text}
    </button>
{/if}
