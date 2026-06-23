<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import FormDialog from "$lib/components/dialogs/FormDialog.svelte";

    const { Story } = defineMeta({
        title: "Dialogs/Form",
        component: FormDialog,
        args: {
            title: "Form Dialog",
            fields: [
                {
                    id: "field1",
                    label: "Enter in all of your personal information",
                    type: "text",
                    placeholder: "insert credit card",
                    required: true,
                },
                {
                    id: "field2",
                    label: "Did you put in your credit card?",
                    type: "select",
                    required: true,
                    options: [
                        { value: "yes", label: "Yes" },
                        { value: "no", label: "No" },
                    ],
                },
            ],
            submitText: "Create",
            cancelText: "Cancel",
        },
        argTypes: {
            title: { control: "text" },
            submitText: { control: "text" },
            cancelText: { control: "text" },
        },
    });
</script>

<Story name="TextFields" />

<Story
    name="WithSelect"
    args={{
        title: "New Rachis",
        fields: [
            {
                id: "title",
                label: "Title",
                type: "text",
                placeholder: "e.g., Twilight Sparkle",
                required: true,
            },
            {
                id: "type",
                label: "Rachis Type",
                type: "select",
                required: true,
                options: [
                    { value: "character", label: "Character" },
                    { value: "location", label: "Location" },
                    { value: "event", label: "Event" },
                    { value: "item", label: "Item" },
                    { value: "note", label: "Note" },
                ],
            },
            {
                id: "folder",
                label: "Folder",
                type: "text",
                placeholder: "e.g., Protagonists/Mane 6",
            },
        ],
        submitText: "Create",
        cancelText: "Cancel",
    }} />
