<script lang="ts">
    import "$lib/styles/app.css";
    import { dialogs } from "$lib/stores/dialog.svelte";

    interface Props {
        /** The title of the dialog */
        title: string;
        /** The message to display in the dialog */
        message: string;
        /** The text to display on the confirm button */
        confirmText: string;
        /** The text to display on the cancel button */
        cancelText: string;
        /** The severity of the dialog */
        severity: "default" | "destructive";
    }

    let { title, message, confirmText, cancelText, severity }: Props = $props();

    /** Resolves the dialog when the user clicks Confirm */
    function handleConfirm(): void {
        dialogs.resolve(true);
    }

    /** Dismisses the dialog when the user clicks Cancel */
    function handleCancel(): void {
        dialogs.dismiss();
    }
</script>

<div
    class="dialog confirm-dialog"
    role="alertdialog"
    aria-labelledby="confirm-title"
    aria-describedby="confirm-message">
    <h2 id="confirm-title" class="text-midnight-violet text-lg font-bold">
        {title}
    </h2>

    <p
        id="confirm-message"
        class="text-midnight-violet text-sm opacity-80 mt-2">
        {message}
    </p>

    <div class="dialog-actions mt-6 flex justify-end gap-x-3">
        <button
            class="btn btn-cancel"
            onclick={handleCancel}
            aria-label={cancelText}>
            {cancelText}
        </button>
        <button
            class="btn btn-confirm"
            class:btn-destructive={severity === "destructive"}
            onclick={handleConfirm}
            aria-label={confirmText}>
            {confirmText}
        </button>
    </div>
</div>

<style>
    .confirm-dialog {
        background: white;
        border-radius: 16px;
        /* padding: 24px 28px */
        padding-inline: 28px;
        padding-block: 24px;
        min-width: 360px;
        max-width: 480px;
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

    .btn-cancel {
        background: var(--color-cornsilk);
        color: var(--color-midnight-violet);
    }
    .btn-cancel:hover {
        background: color-mix(
            in oklch,
            var(--color-cornsilk),
            var(--color-lilac) 15%
        );
    }

    .btn-confirm {
        background: var(--color-lilac);
        color: white;
    }
    .btn-confirm:hover {
        background: color-mix(in oklch, var(--color-lilac), black 10%);
    }

    .btn-destructive {
        background: #dc2626;
    }
    .btn-destructive:hover {
        background: #b91c1c;
    }
</style>
