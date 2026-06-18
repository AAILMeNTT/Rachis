<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import Button from "$lib/components/layout/Button.svelte";

    const { Story } = defineMeta({
        title: "UI/Button",
        component: Button,
        args: {
            text: "Button",
            type: "primary",
        },
        argTypes: {
            text: { control: "text" },
            type: {
                options: ["primary", "secondary", "tertiary", "disabled"],
            },
        },
    });
</script>

<Story
    name="Primary"
    args={{
        text: "Button",
        type: "primary",
    }} />

<Story
    name="Secondary"
    args={{
        text: "Button",
        type: "secondary",
    }} />

<Story
    name="Tertiary"
    args={{
        text: "Button",
        type: "tertiary",
    }} />

<Story
    name="Disabled"
    args={{
        text: "Button",
        type: "disabled",
    }} />
