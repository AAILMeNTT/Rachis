<script module lang="ts">
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import BaseWidget from "$lib/components/widgets/BaseWidget.svelte";
    import { WidgetType } from "$lib/types/WidgetType";
    import { stubLeaf } from "./stubs.svelte";

    const { Story } = defineMeta({
        title: "Widgets/Base",
        component: BaseWidget,
        args: {
            leaf: stubLeaf(),
            isLoading: false,
            error: null,
        },
        argTypes: {
            leaf: { control: "object" },
            isLoading: { control: "boolean" },
            error: { control: "text" },
        },
    });
</script>

<Story name="Base Widget" />

<Story
    name="Loading"
    args={{
        leaf: stubLeaf({ widget_type: WidgetType.Editor }),
        isLoading: true,
    }} />

<Story
    name="Error"
    args={{
        leaf: stubLeaf({ widget_type: WidgetType.Editor }),
        error: "Something went wrong.",
    }} />
