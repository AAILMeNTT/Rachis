<!--
    @component
    BaseWidget is the shell for all widgets, providing a consistent title bar
    and close button (...soon™️).

    If you want to create a new Widget, just create a new Svelte component in
    this directory; it will automatically be handled by this wrapper component
    so long as:

    1. it matches the glob pattern `*Widget.svelte`; and
    2. it takes in a `leaf` prop of type `Leaf` (see the `types` directory).

    Go crazy.
-->
<script lang="ts">
    import type { Leaf } from "$lib/types/Leaf";
    import { resolveWidget } from "$lib/registries/widget_registry.svelte";

    interface Props {
        /** Text to show in the widget's title bar */
        leaf: Leaf;
        /** Whether to show a loading indicator over the widget body */
        isLoading?: boolean;
        /** If set, shows an error banner inside the widget */
        error?: string | null;
        /** Called when the user clicks the close button */
        onClose?: () => void;
    }

    let { leaf, isLoading, error, onClose }: Props = $props();

    // This part was REALLY WEIRD
    /** Gets the pairing component based off the `widget_type` field in `leaf` */
    let Widget = $derived(resolveWidget(leaf.widget_type));

    async function onCloseDefault(): Promise<void> {}
</script>

<div
    class="base-widget
    flex flex-col h-full bg-white rounded-lg overflow-hidden border-cornsilk border-2">
    <!-- Title Bar -->
    <div
        class="title-bar
        flex items-center justify-between px-4 py-2.5 bg-cornsilk select-none">
        <span>{leaf.widget_type}</span>

        <button
            class="close-btn
            cursor-pointer"
            onclick={onClose || onCloseDefault}>
            ×
        </button>
    </div>

    <!-- Error Banner -->
    {#if error}
        <div><p>{error}</p></div>
    {/if}

    <!-- Widget Body -->
    <div class="widget-body" class:is-loading={isLoading}>
        {#if isLoading}
            <!-- TODO: Replace with, like... slowly ebbing white pulse? oslt -->
        {/if}

        <!-- If `Widget` is a valid component (i.e., if the widget_type of leaf pairs with a valid Svelte component), render it -->
        {#if Widget}
            <Widget {leaf} />
        {:else}
            <div>
                <p>Unknown widget: {leaf.widget_type}</p>
                <p>Leaf: {JSON.stringify(leaf, null, 4)}</p>
            </div>
        {/if}
    </div>
</div>
