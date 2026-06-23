<script lang="ts">
    import "$lib/styles/app.css";
    import { dialogs } from "$lib/stores/dialog.svelte";
    import ConfirmDialog from "$lib/components/dialogs/ConfirmDialog.svelte";
    import PromptDialog from "$lib/components/dialogs/PromptDialog.svelte";
    import AlertDialog from "$lib/components/dialogs/AlertDialog.svelte";
    import FormDialog from "$lib/components/dialogs/FormDialog.svelte";
    import NotifyDialog from "$lib/components/dialogs/NotifyDialog.svelte";

    let severity: string = $derived(
        `notification-${dialogs.notification?.severity ?? "info"}`
    );

    /** Handles the Escape key to dismiss the dialog */
    function handleKeydown(event: KeyboardEvent): void {
        if (event.key === "Escape" && dialogs.active) {
            dialogs.dismiss();
        }
    }

    /** Dismisses the dialog when the user clicks outside of it */
    function handleBackdropClick(event: MouseEvent): void {
        const target: HTMLElement = event.target as HTMLElement;
        if (target.classList.contains("dialog-backdrop")) {
            dialogs.dismiss();
        }
    }
</script>

<!-- Handle key presses anywhere in the active window -->
<svelte:window onkeydown={handleKeydown} />

{#if dialogs.active}
    <!-- Render a backdrop that covers the entire screen and handles clicks outside of the dialog -->
    <div
        class="dialog-backdrop
        fixed inset-0 flex items-center justify-center z-1000"
        onclick={handleBackdropClick}
        role="presentation"
        aria-hidden="true">
        <div class="dialog-container">
            <!-- Render a Dialog based off the active Dialog's type -->
            {#if dialogs.active.type === "confirm"}
                <ConfirmDialog
                    title={dialogs.active.title}
                    message={dialogs.active.message}
                    confirmText={dialogs.active.confirmText ?? "Confirm"}
                    cancelText={dialogs.active.cancelText ?? "Cancel"}
                    severity={dialogs.active.severity ?? "default"} />
            {:else if dialogs.active.type === "prompt"}
                <PromptDialog
                    title={dialogs.active.title}
                    label={dialogs.active.label}
                    placeholder={dialogs.active.placeholder}
                    defaultValue={dialogs.active.defaultValue}
                    submitText={dialogs.active.submitText ?? "Submit"}
                    cancelText={dialogs.active.cancelText ?? "Cancel"} />
            {:else if dialogs.active.type === "form"}
                <FormDialog
                    title={dialogs.active.title}
                    fields={dialogs.active.fields}
                    submitText={dialogs.active.submitText ?? "Create"}
                    cancelText={dialogs.active.cancelText ?? "Cancel"} />
            {:else if dialogs.active.type === "alert"}
                <AlertDialog
                    title={dialogs.active.title}
                    message={dialogs.active.message}
                    okText={dialogs.active.okText ?? "OK"} />
            {/if}
        </div>
    </div>
{/if}

{#if dialogs.notification}
    <div
        class={`notification-toast ${severity}`}
        role="status"
        aria-live="polite">
        <NotifyDialog />
    </div>
{/if}

<!-- TODO: just take all this crap out i hate style tag so much just get rid of all of it -->
<style>
    .dialog-backdrop {
        background: rgba(29, 11, 49, 0.4);
    }

    /* TODO: Tailwind-ify this */
    .notification-toast {
        position: fixed;
        bottom: 24px;
        right: 24px;
        z-index: 1100;
        min-width: 280px;
        max-width: 400px;
        padding: 14px 18px;
        border-radius: 12px;
        box-shadow: 0 4px 20px rgba(29, 11, 49, 0.12);
        display: flex;
        align-items: flex-start;
        justify-content: space-between;
        gap: 12px;
    }

    /* TODO: Extract these colours to the main CSS file */
    .notification-info {
        background: white;
        border-left: 4px solid var(--color-lilac);
    }

    .notification-success {
        background: white;
        border-left: 4px solid #16a34a;
    }

    .notification-error {
        background: white;
        border-left: 4px solid #dc2626;
    }

    .notification-content {
        flex: 1;
    }

    .notification-title {
        font-size: 14px;
        font-weight: 600;
        color: var(--color-midnight-violet);
    }

    .notification-message {
        font-size: 12px;
        color: var(--color-midnight-violet);
        opacity: 0.7;
        margin-top: 2px;
    }

    .notification-close {
        background: none;
        border: none;
        cursor: pointer;
        font-size: 18px;
        color: var(--color-midnight-violet);
        opacity: 0.4;
        padding: 0 2px;
        line-height: 1;
    }
    .notification-close:hover {
        opacity: 0.8;
    }
</style>
