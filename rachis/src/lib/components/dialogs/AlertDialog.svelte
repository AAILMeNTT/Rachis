<script lang="ts">
    import "$lib/styles/app.css";
    import { dialogs } from "$lib/stores/dialog.svelte";

    interface Props {
        /** The title of the dialog */
        title: string;
        /** The message to display in the dialog */
        message: string;
        /** The text to display on the OK button */
        okText: string;
    }

    let { title, message, okText }: Props = $props();

    /** Resolves the dialog when the user clicks the OK button */
    function handleOk(): void {
        dialogs.resolve(undefined);
    }
</script>

<div
    class="dialog alert-dialog
    bg-white rounded-2xl"
    role="alertdialog"
    aria-labelledby="alert-title"
    aria-describedby="alert-message">
    <h2 id="alert-title" class="text-midnight-violet text-lg font-bold">
        {title}
    </h2>

    <p id="alert-message" class="text-midnight-violet text-sm opacity-80 mt-2">
        {message}
    </p>

    <div class="dialog-actions mt-6 flex justify-end">
        <button class="btn btn-ok" onclick={handleOk} aria-label={okText}>
            {okText}
        </button>
    </div>
</div>

<style>
    /* TODO: for some reason i cant transcribe these to Tailwind classes so figure out what's goin on there as asap as possible */
    .alert-dialog {
        background: white;
        border-radius: 16px;
        /* padding: 24px 28px */
        padding-inline: 28px;
        padding-block: 24px;
        min-width: 320px;
        max-width: 440px;
        box-shadow: 0 8px 32px rgba(29, 11, 49, 0.15);
    }

    .dialog-actions button {
        cursor: pointer;
        /*padding: 8px 20px;*/
        padding-inline: 20px;
        padding-block: 8px;
        border-radius: 10px;
        font-size: 14px;
        font-weight: 500;
        border: none;
        transition: all 0.15s;
    }

    .btn-ok {
        background: var(--color-lilac);
        color: white;
    }
    .btn-ok:hover {
        background: color-mix(in oklch, var(--color-lilac), black 10%);
    }
</style>
