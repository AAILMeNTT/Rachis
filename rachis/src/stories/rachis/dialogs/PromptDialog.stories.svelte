<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import PromptDialog from "$lib/components/dialogs/PromptDialog.svelte";

    const { Story } = defineMeta({
        title: "Dialogs/Prompt",
        component: PromptDialog,
        args: {
            title: "Prompt Box",
            label: "Enter something into the box... or else.",
            placeholder: "Do it. You have no choice.",
            defaultValue: "",
            submitText: "Submit",
            cancelText: "Cancel",
        },
        argTypes: {
            title: { control: "text" },
            label: { control: "text" },
            placeholder: { control: "text" },
            defaultValue: { control: "text" },
            submitText: { control: "text" },
            cancelText: { control: "text" },
        },
    });
</script>

<Story name="Prompt" />
