<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import AlertDialog from "$lib/components/dialogs/AlertDialog.svelte";

    const { Story } = defineMeta({
        title: "Dialogs/Alert",
        component: AlertDialog,
        args: {
            title: "Notification",
            message:
                "You have been notified. Confirm that you have been notified.",
            okText: "OK",
        },
        argTypes: {
            title: { control: "text" },
            message: { control: "text" },
            okText: { control: "text" },
        },
    });
</script>

<Story name="Alert" />
