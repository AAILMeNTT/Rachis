<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import ConfirmDialog from "$lib/components/dialogs/ConfirmDialog.svelte";

    const { Story } = defineMeta({
        title: "Dialogs/Confirm",
        component: ConfirmDialog,
        args: {
            title: "Confirm Your Action",
            message:
                "Please confirm that you would like to carry through with this action.",
            confirmText: "Confirm",
            cancelText: "Cancel",
            severity: "default",
        },
        argTypes: {
            title: { control: "text" },
            message: { control: "text" },
            confirmText: { control: "text" },
            cancelText: { control: "text" },
            severity: {
                options: ["default", "destructive"],
            },
        },
    });
</script>

<Story name="Confirm" />

<Story
    name="ConfirmDanger"
    args={{
        message: "This is an irreversible action! Are you certain?",
        severity: "destructive",
    }} />
