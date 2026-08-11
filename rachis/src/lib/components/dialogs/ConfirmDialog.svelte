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
    class="dialog confirm-dialog
    bg-white px-7 py-6 rounded-2xl min-w-90 max-w-120 shadow-[0_8px_32px_rgba(29,11,49,0.15)]"
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

    <div class="dialog-actions
    mt-6 flex justify-end gap-x-3
    *:cursor-pointer *:px-5 *:py-2 *:rounded-[10px] *:text-sm *:font-medium *:border-none">
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
    /* TODO: All this shit is gonna change cause i am gonna use really cool themes just you wait */
    .dialog-actions button {
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
