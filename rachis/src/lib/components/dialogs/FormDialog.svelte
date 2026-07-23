<script lang="ts">
    import "$lib/styles/app.css";
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

    /** The layout of the form fields */
    let layout = $derived(fields.map((f: FormField) => f.layout ?? "vertical"));

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
        if (event.key === "Enter" && canSubmit) handleSubmit();
    }
</script>

<div
    class="dialog form-dialog
    bg-white rounded-2xl px-7 py-6 min-w-100 max-w-130 shadow-[0_8px_32px_rgba(29,11,49,0.15)]"
    role="dialog"
    aria-labelledby="form-title">
    <h2 id="form-title" class="text-midnight-violet text-lg font-bold">
        {title}
    </h2>

    <div
        class="form-fields flex flex-col gap-4 mt-4"
        onkeydown={handleKeydown}
        role="none">
        {#each fields as field, i (field.id)}
            <div
                class="form-field flex {layout[i] === 'horizontal' ?
                    'flex-row items-start gap-x-4'
                :   'flex-col'}">
                <label
                    for={`form-${field.id}`}
                    class="block text-midnight-violet text-sm mb-1.5">
                    {field.label}
                    {#if field.required}
                        <span class="text-red-500" aria-label="required"
                            >*</span>
                    {/if}
                </label>

                {#if field.type === "info"}
                    <!-- The "info" field type is used for displaying information to the user, not for input, and it doesn't need a display since the label does that already -->
                {:else if field.type === "select" && field.options}
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
                                focus:outline-none focus:border-lilac focus:ring-1 focus:ring-lilac hover:cursor-pointer">
                        {#each field.options as option}
                            <option value={option.value}>{option.label}</option>
                        {/each}
                    </select>
                {:else if field.type === "radio" && field.options}
                    <div
                        class="flex gap-2 {layout[i] === 'horizontal' ?
                            'flex-row'
                        :   'flex-col'}"
                        role="radiogroup"
                        aria-label={field.label}>
                        {#each field.options as option}
                            <!-- TODO: Add some way to deselect the option -->
                            <label
                                class="radio-option
                                flex items-center gap-1.5 cursor-pointer text-sm bg-white text-midnight-violet px-3 py-1.5 border border-wisteria rounded-lg transition-all duration-0.15s has-checked:bg-lilac has-checked:text-white has-checked:border-white/0">
                                <input
                                    type="radio"
                                    name={`form-${field.id}`}
                                    value={option.value}
                                    checked={values[field.id] === option.value}
                                    onchange={(e: Event): void => {
                                        updateValue(
                                            field.id,
                                            (e.target as HTMLInputElement).value
                                        );
                                    }}
                                    class="radio-input appearance-none absolute" />
                                <span class="select-none pointer-events-none"
                                    >{option.label}</span>
                            </label>
                        {/each}
                    </div>
                {:else if field.type === "text"}
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

    <div
        class="mt-6 flex justify-end gap-x-3
        *:cursor-pointer *:px-5 *:py-2 *:rounded-[10px] *:font-medium *:text-sm *:transition-all *:duration-150">
        <!-- TODO: Add in the whole like -100/-200/-300 things for the colours -->
        <button
            class="bg-cornsilk text-midnight-violet hover:bg-cornsilk-300"
            onclick={handleCancel}
            aria-label={cancelText}>{cancelText}</button
        ><button
            class="btn-submit bg-lilac text-white disabled:opacity-50 disabled:cursor-not-allowed"
            onclick={handleSubmit}
            disabled={!canSubmit}
            aria-label={submitText}>{submitText}</button>
    </div>
</div>
