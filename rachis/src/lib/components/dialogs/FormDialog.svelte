<script lang="ts">
    // import "$lib/styles/app.css";
    import { dialogs } from "$lib/stores/dialog.svelte";
    import type { FormField } from "$lib/types/Dialog";

    interface Props {
        /** The title of the form dialog */
        title: string;
        /** The fields to display in the form */
        fields: FormField[];
        /** The text to display on the submit button */
        submitText: string;
        /** The text to display on the cancel button */
        cancelText: string;
    }

    let { title, fields, submitText, cancelText }: Props = $props();

    /** Tracks the current value of each form field */
    let values: Record<string, string> = $state<Record<string, string>>({});

    // Added this so that Svelte would stop crying on some "waaaa waaaa fields is static :(((" ts
    $effect((): void => {
        Object.fromEntries(
            fields.map((f: FormField): [string, string] => [
                f.id,
                f.defaultValue ?? "",
            ])
        );
    });

    /** Whether all required fields have non-empty values */
    let canSubmit: boolean = $derived(
        fields
            // Get all required fields
            .filter((f: FormField): boolean => f.required ?? false)
            // Verify they all have some form of content
            .every(
                (f: FormField): boolean =>
                    (values[f.id] ?? "").trim().length > 0
            )
    );

    /** Updates a field's value on input */
    function updateValue(id: string, value: string): void {
        values[id] = value;
    }

    /** Resolves the form dialog with the current values */
    function handleSubmit(): void {
        dialogs.resolve(values);
    }

    /** Dismisses the form if the cancel button is clicked */
    function handleCancel(): void {
        dialogs.dismiss();
    }

    /** Handles keyboard input */
    function handleKeydown(event: KeyboardEvent): void {
        // If Enter is pressed and the form can be submitted, handle it
        if (event.key === "Enter" && canSubmit) {
            handleSubmit();
        }
    }
</script>

<div class="dialog form-dialog" role="dialog" aria-labelledby="form-title">
    <h2 id="form-title" class="text-midnight-violet text-lg font-bold">
        {title}
    </h2>

    <div class="form-fields" onkeydown={handleKeydown} role="none">
        {#each fields as field (field.id)}
            <div class="form-field">
                <label
                    for={`form-${field.id}`}
                    class="block text-midnight-violet text-sm mb-1.5">
                    {field.label}
                    {#if field.required}
                        <span class="text-red-500" aria-label="required"
                            >*</span>
                    {/if}
                </label>

                {#if field.type === "select" && field.options}
                    <select
                        id={`form-${field.id}`}
                        value={values[field.id]}
                        onchange={(e: Event): void => {
                            updateValue(
                                field.id,
                                (e.target as HTMLSelectElement).value
                            );
                        }}
                        class="w-full px-3 py-2 rounded-lg border border-wisteria/30
                               text-midnight-violet text-sm bg-white
                               focus:outline-none focus:border-lilac focus:ring-1 focus:ring-lilac">
                        {#each field.options as option}
                            <option value={option.value}>{option.label}</option>
                        {/each}
                    </select>
                {:else}
                    <input
                        id={`form-${field.id}`}
                        type="text"
                        value={values[field.id]}
                        oninput={(e: Event): void => {
                            updateValue(
                                field.id,
                                (e.target as HTMLInputElement).value
                            );
                        }}
                        placeholder={field.placeholder}
                        class="w-full px-3 py-2 rounded-lg border border-wisteria/30
                               text-midnight-violet text-sm
                               focus:outline-none focus:border-lilac focus:ring-1 focus:ring-lilac" />
                {/if}
            </div>
        {/each}
    </div>

    <div class="dialog-actions mt-6 flex justify-end gap-x-3">
        <button
            class="btn btn-cancel"
            onclick={handleCancel}
            aria-label={cancelText}>
            {cancelText}
        </button>
        <button
            class="btn btn-submit"
            onclick={handleSubmit}
            disabled={!canSubmit}
            aria-label={submitText}>
            {submitText}
        </button>
    </div>
</div>

<style>
    .form-dialog {
        background: white;
        border-radius: 16px;
        /* padding: 24px 28px */
        padding-inline: 28px;
        padding-block: 24px;
        min-width: 400px;
        max-width: 520px;
        box-shadow: 0 8px 32px rgba(29, 11, 49, 0.15);
    }

    .form-fields {
        display: flex;
        flex-direction: column;
        gap: 16px;
        margin-top: 16px;
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

    select:hover {
        cursor: pointer;
    }
</style>
