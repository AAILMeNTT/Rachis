<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import NotifyDialog from "$lib/components/dialogs/NotifyDialog.svelte";

    const { Story } = defineMeta({
        title: "Dialogs/Notify",
        component: NotifyDialog,
        args: {
            title: "Notify Dialog",
        },
        argTypes: {
            title: { control: "text" },
        },
    });

    // TODO: figure out how to test this without complete reliance on the store
</script>

<Story name="Notify"></Story>
