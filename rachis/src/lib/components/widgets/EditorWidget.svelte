<!--
    @component
    EditorWidget is used to display the main editing area for users to write in.

    Rendered by the BaseWidget component; this file simply describes the child
    data to be rendered within the BaseWidget shell.
-->
<script lang="ts">
    interface Props {
        /** Text to show in the widget's title bar */
        title: string;
        /** The editor's content */
        content: string;
    }

    let { title, content }: Props = $props();

    /**
     * TODO: ddo something
     *
     * @param content
     */
    async function onSave(content: string): Promise<void> {}

    /**
     * TODO: also here do osotmehing
     */
    async function onClose(): Promise<void> {}
</script>

<!-- TODO: Replace with CodeMirror editor -->
<p>Content: {content}</p>
