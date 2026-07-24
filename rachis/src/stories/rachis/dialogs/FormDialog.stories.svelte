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
                    layout: "vertical",
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
                    layout: "vertical",
                },
            ],
            submitText: "Done",
            cancelText: "Cancel",
        },
        argTypes: {
            title: { control: "text" },
            fields: { control: "object" },
            submitText: { control: "text" },
            cancelText: { control: "text" },
        },
    });
</script>

<Story name="TextFields" />

<Story
    name="Select"
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

<Story
    name="Vertical Radio"
    args={{
        title: "New Rachis",
        fields: [
            {
                id: "name",
                label: "Rachis Name",
                type: "text",
                placeholder: "Enter the rachis name",
                required: true,
                layout: "vertical",
            },
            {
                id: "type",
                label: "Rachis Type",
                type: "radio",
                required: true,
                layout: "vertical",
                options: [
                    { value: "character", label: "Character" },
                    { value: "location", label: "Location" },
                    { value: "event", label: "Event" },
                    { value: "item", label: "Item" },
                    { value: "note", label: "Note" },
                ],
            },
        ],
        submitText: "Create",
        cancelText: "Cancel",
    }} />

<Story
    name="Horizontal Radio"
    args={{
        title: "New Rachis",
        fields: [
            {
                id: "name",
                label: "Rachis Name",
                type: "text",
                placeholder: "Enter the rachis name",
                required: true,
                layout: "horizontal",
            },
            {
                id: "type",
                label: "Rachis Type",
                type: "radio",
                required: true,
                layout: "horizontal",
                options: [
                    { value: "character", label: "Character" },
                    { value: "location", label: "Location" },
                    { value: "event", label: "Event" },
                    { value: "item", label: "Item" },
                    { value: "note", label: "Note" },
                ],
            },
        ],
        submitText: "Create",
        cancelText: "Cancel",
    }} />

<Story
    name="Mixed Radio"
    args={{
        title: "New Rachis",
        fields: [
            {
                id: "name",
                label: "Rachis Name",
                type: "text",
                placeholder: "Enter the rachis name",
                required: true,
                layout: "vertical",
            },
            {
                id: "type",
                label: "Rachis Type",
                type: "radio",
                required: true,
                layout: "horizontal",
                options: [
                    { value: "character", label: "Character" },
                    { value: "location", label: "Location" },
                    { value: "event", label: "Event" },
                    { value: "item", label: "Item" },
                    { value: "note", label: "Note" },
                ],
            },
        ],
        submitText: "Create",
        cancelText: "Cancel",
    }} />

<Story
    name="Info"
    args={{
        title: "New Rachis",
        fields: [
            {
                id: "just_letting_you_know",
                label: "This isn't really a form I'm just like letting you know",
                type: "info",
            },
            {
                id: "oh_yeah",
                label: "Oh yeah yeah yeah",
                type: "info",
            },
        ],
        submitText: "Create",
        cancelText: "Cancel",
    }} />
