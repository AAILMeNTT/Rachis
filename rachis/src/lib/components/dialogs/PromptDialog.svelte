<script lang="ts">
    import "$lib/styles/app.css";
    import { dialogs } from "$lib/stores/dialog.svelte";

    interface Props {
        /** The title of the prompt dialog */
        title: string;
        /** The label for the input field */
        label: string;
        /** The placeholder text for the input field */
        placeholder?: string;
        /** The default value for the input field */
        defaultValue?: string;
        /** The text for the submit button */
        submitText: string;
        /** The text for the cancel button */
        cancelText: string;
    }

    let {
        title,
        label,
        placeholder,
        defaultValue,
        submitText,
        cancelText,
    }: Props = $props();

    let inputValue: string = $derived<string>(defaultValue ?? "");

    /** Resolves the dialog with the input value */
    function handleSubmit(): void {
        dialogs.resolve(inputValue.trim());
    }

    /** Cancels the dialog */
    function handleCancel(): void {
        dialogs.dismiss();
    }

    /** Handles the keydown event, submitting on Enter if the input value is not empty */
    function handleKeydown(event: KeyboardEvent): void {
        if (event.key === "Enter" && inputValue.trim()) {
            handleSubmit();
        }
    }
</script>

<div
    class="dialog prompt-dialog"
    role="dialog"
    aria-labelledby="prompt-title"
    aria-describedby="prompt-label">
    <h2 id="prompt-title" class="text-midnight-violet text-lg font-bold">
        {title}
    </h2>

    <label
        id="prompt-label"
        for="dialog-input"
        class="block text-midnight-violet text-sm mt-4 mb-1.5">
        {label}
    </label>

    <input
        id="dialog-input"
        type="text"
        bind:value={inputValue}
        {placeholder}
        onkeydown={handleKeydown}
        class="w-full px-3 py-2 rounded-lg border border-wisteria/30
               text-midnight-violet text-sm
               focus:outline-none focus:border-lilac focus:ring-1 focus:ring-lilac" />

    <div class="dialog-actions mt-5 flex justify-end gap-x-3">
        <button
            class="btn btn-cancel"
            onclick={handleCancel}
            aria-label={cancelText}>
            {cancelText}
        </button>
        <button
            class="btn btn-submit"
            onclick={handleSubmit}
            disabled={!inputValue.trim()}
            aria-label={submitText}>
            {submitText}
        </button>
    </div>
</div>

<style>
    .prompt-dialog {
        background: white;
        border-radius: 16px;
        padding: 24px 28px;
        min-width: 360px;
        max-width: 480px;
        box-shadow: 0 8px 32px rgba(29, 11, 49, 0.15);
    }

    .dialog-actions button {
        cursor: pointer;
        padding: 8px 20px;
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

    .btn-submit {
        background: var(--color-lilac);
        color: white;
    }
    .btn-submit:hover:not(:disabled) {
        background: color-mix(in oklch, var(--color-lilac), black 10%);
    }
    .btn-submit:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    input::placeholder {
        color: var(--color-midnight-violet);
        opacity: 0.35;
    }
</style>
