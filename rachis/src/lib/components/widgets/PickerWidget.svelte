<!--
    @component
    PickerWidget is the default Widget used to display a selection of other Widgets the user may choose from.

    Rendered by the BaseWidget component; this file simply describes the child
    data to be rendered within the BaseWidget shell.
-->
<script lang="ts">
    interface Props {
        /** Text to show in the widget's title bar */
        title: string;
    }

    let { title }: Props = $props();

    /**
     * TODO: also here do osotmehing
     */
    async function onClose(): Promise<void> {}
</script>

<div class="picker">
    <div></div>
    <div>
        <div></div>
        <div></div>
        <div></div>
        <div></div>
    </div>
</div>
