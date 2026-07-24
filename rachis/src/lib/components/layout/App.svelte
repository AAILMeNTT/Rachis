<script lang="ts">
    import "$lib/styles/app.css";
    import Landing from "$lib/components/layout/Landing.svelte";
    import DialogShell from "$lib/components/dialogs/DialogShell.svelte";
</script>

<div
    class="app
    w-480 h-270 bg-cornsilk flex flex-col flex-nowrap items-start content-stretch justify-start">
    <Landing />
</div>

<!--
TODO: HUGE PROBLEM(!!) (not really just annoying)

For some reason the purple background like gets a yellow tint to it whenever certain screen things happen. Notably when I try to take a screenshot (on Linux - don't know about other OS environments) and when the dialog boxes appear. Perhaps it's when the screen darkens to any meaningful capacity?
-->

<!-- TODO: also make the app a11y cause right now its pretty bad -->

<DialogShell />
