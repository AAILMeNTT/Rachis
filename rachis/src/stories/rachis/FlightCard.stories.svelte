<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import FlightCard from "$lib/components/layout/FlightCard.svelte";

    const { Story } = defineMeta({
        title: "Flights/FlightCard",
        component: FlightCard,
        argTypes: {
            title: { control: "text" },
            isLastActive: { control: "boolean" },
            isFavorite: { control: "boolean" },
        },
    });
</script>

<Story
    name="MLP: Ad Eternum"
    args={{
        title: "MLP: Ad Eternum",
        isLastActive: false,
        isFavorite: false,
    }} />

<Story
    name="Light as a Feather"
    args={{
        title: "Light as a Feather",
        isLastActive: true,
        isFavorite: false,
    }} />

<Story
    name="SE7ENFOLD"
    args={{
        title: "SE7ENFOLD",
        isLastActive: false,
        isFavorite: true,
    }} />

<Story
    name="Sonder"
    args={{
        title: "Sonder",
        isLastActive: true,
        isFavorite: true,
    }} />
