import type {
    Dialog,
    ConfirmDialog,
    PromptDialog,
    AlertDialog,
    FormDialog,
    FormField,
    NotifyDialog,
} from "$lib/types/Dialog";

/**
 * Manages all dialogs and notifications in the app.
 *
 * Each Dialog that requires user input creates a Promise, saves its `resolve`
 * function to `#_resolve`, and sets `this.active` to the respective dialog's
 * config. When the user clicks a button in the dialog, `dialogs.resolve(value)`
 * is called, which:
 *
 * 1. Calls the saved `resolve` — the Promise fulfills, your `await` unblocks
 * 2. Clears `this.active` — the dialog shell unmounts
 *
 * The result is that calling code reads top-to-bottom even though dialogs
 * are inherently asynchronous.
 *
 * ## Examples
 *
 * ```ts
 * // YourComponent.svelte
 *
 * // Confirm dialog returns a Promise that resolves to `true` or `false`
 * const accept = await dialogs.confirm(
 *     "FREE $200,000",
 *     "Claim your FREE $200,000 RIGHT NOW?!?",
 *     {
 *         "confirmText": "wow! wonderful"
 *         "cancelText": "this is the deny button"
 *     }
 * );
 * if (accept) { user.isStupid == true };
 *
 * // Prompt dialog returns a Promise that resolves to the user's input
 * const ssn = await dialogs.prompt("Okay you have to tell me your social security number then");
 * if (ssn) { user.setSSN(ssn); }
 *
 * // Form dialog returns a Promise that resolves to the user's input for all fields
 * const form = await dialogs.form("okay awesome now fill in this stuff", [
 *     { id: "favouriteColour", label: "what your favourite colour", type: "text", required: true }
 *     {
 *         id: "element",
 *         label: "what element would you bend",
 *         type: "select",
 *         options: [
 *             { value: "water", label: "probably a water bender" },
 *             { value: "earth", label: "probably a earthbender" },
 *             { value: "fire", label: "probabyl a fire bender" },
 *             { value: "air", label: "a Air bedner" },
 *         ]
 *     }
 * ]);
 * if (form) { ... }
 *
 * // Alert dialog returns `void`, as you might expect
 * await dialogs.alert("Snake Warning", "you are a snake now", "okay")
 * // Function continues when the user clicks the ok button
 *
 * // Notify dialog just puts a little guy in the bottom right :)
 * dialogs.notify("what is the 🐕️ doingn", { severity: "warning" })
 * // Function continues immediately, it's not await(!!)
 * ```
 */
class DialogStore {
    /** The currently active blocking dialog; `null` when nothing is shown. */
    active: Dialog | null = $state<Dialog | null>(null);

    /** The currently visible notification toast; `null` when nothing is shown. */
    notification: NotifyDialog | null = $state<NotifyDialog | null>(null);

    /**
     * Saved Promise resolve function for the active dialog.
     *
     * Set when a dialog opens (inside `new Promise()`), called when the user
     * clicks a button, cleared when the dialog closes.
     *
     * DON'T TOUCH UNLESS YOU ARE THIS FILE
     */
    #_resolve: ((value: any) => void) | null = null;

    // =========================================================================
    // Blocking dialogs
    // =========================================================================

    /**
     * Shows a two-button confirmation dialog.
     *
     * This dialog is blocking, so call it with `await` or face felony charges
     *
     * @param title The dialog title
     * @param message The message to display
     * @param options Optional confirmation and cancel text and severity
     *
     * @returns `true` if the user confirmed, `false` otherwise.
     *
     * @example
     * ```ts
     * const ok = await dialogs.confirm(
     *     "Delete Flight?",
     *     `This will delete "${flight.name}" permanently.`,
     *     { confirmText: "Delete", severity: "destructive" }
     * );
     * if (ok) { await landingStore.remove(flight.id); }
     * ```
     */
    confirm(
        title: string,
        message: string,
        options?: Partial<
            Pick<ConfirmDialog, "confirmText" | "cancelText" | "severity">
        >
    ): Promise<boolean> {
        // TODO: Destructive ConfirmDialogs should require the user to press and hold on the confirm button for 0.8-1.5 seconds to ensure intention (perhaps togglable in user settings?)
        return new Promise<boolean>((resolve) => {
            this.#_resolve = resolve;
            this.active = {
                type: "confirm",
                title,
                message,
                confirmText: options?.confirmText ?? "Confirm",
                cancelText: options?.cancelText ?? "Cancel",
                severity: options?.severity ?? "default",
            };
        });
    }

    /**
     * Shows a single-field text input dialog; the input's Submit button is
     * disabled if any required field is empty.
     *
     * This dialog is blocking, so call it with `await` or become instantly bankrupt
     *
     * @param title The dialog title
     * @param label The label for the input field
     * @param options Optional placeholder, default value, and submit and cancel text
     *
     * @returns The input string, or `null` if the user cancelled.
     *
     * @example
     * ```ts
     * const name = await dialogs.prompt("Rename Flight", "New name?");
     * if (name) { await landingStore.update(id, name, path); }
     * ```
     */
    prompt(
        title: string,
        label: string,
        options?: Partial<
            Pick<
                PromptDialog,
                "placeholder" | "defaultValue" | "submitText" | "cancelText"
            >
        >
    ): Promise<string | null> {
        return new Promise<string | null>((resolve) => {
            this.#_resolve = resolve;
            this.active = {
                type: "prompt",
                title,
                label,
                placeholder: options?.placeholder,
                defaultValue: options?.defaultValue,
                submitText: options?.submitText ?? "Submit",
                cancelText: options?.cancelText ?? "Cancel",
            };
        });
    }

    /**
     * Shows a multi-field form dialog with text inputs and/or select dropdowns;
     * the Submit button is disabled until all `required` fields have non-empty
     * values.
     *
     * This dialog is blocking, so call it with `await` or I sick my vultures on you
     *
     * @param title The dialog title
     * @param fields An array of `FormField` definitions
     * @param options Optional submit and cancel text
     *
     * @returns a `Record<string, string>` mapping field IDs to their values,
     * or `null` if the user cancelled.
     *
     * @example
     * ```ts
     * // With text inputs only:
     * const result = await dialogs.form("New Flight", [
     *     { id: "name", label: "Project Name", type: "text", required: true },
     *     { id: "path", label: "File Path", type: "text" },
     * ]);
     * if (result) {
     *     doSomeWhatever(result.name, result.path);
     * }
     *
     * // With a select dropdown:
     * const result = await dialogs.form("New Rachis", [
     *     { id: "title", label: "Title", type: "text", required: true },
     *     { id: "type", label: "Type", type: "select", options: [
     *         { value: "character", label: "Character" },
     *         { value: "location", label: "Location" },
     *     ]},
     * ]);
     * if (result) {
     *     insaneBackflipFunction(result.title, result.type[0], result.type[1]);
     * }
     * ```
     */
    form(
        title: string,
        fields: FormField[],
        options?: Partial<Pick<FormDialog, "submitText" | "cancelText">>
    ): Promise<Record<string, string> | null> {
        return new Promise<Record<string, string> | null>((resolve) => {
            this.#_resolve = resolve;
            this.active = {
                type: "form",
                title,
                fields,
                submitText: options?.submitText ?? "Submit",
                cancelText: options?.cancelText ?? "Cancel",
            };
        });
    }

    /**
     * Shows an informational dialog with a single OK button.
     *
     * Use for errors, success confirmations, or any message that just needs an acknowledgement.
     *
     * @param title The dialog title
     * @param message The message to display
     * @param options Optional OK button text
     *
     * @example
     * ```ts
     * await dialogs.alert("Under Arrest Notification", "you are now under arrest", "oh no button");
     * ```
     */
    alert(
        title: string,
        message: string,
        options?: Partial<Pick<AlertDialog, "okText">>
    ): Promise<void> {
        return new Promise<void>((resolve) => {
            this.#_resolve = resolve;
            this.active = {
                type: "alert",
                title,
                message,
                okText: options?.okText ?? "OK",
            };
        });
    }

    /**
     * Shows a brief notification toast that auto-dismisses.
     *
     * Default duration is 8000ms. Set `duration: 0` to make it sticky
     * (user must manually dismiss).
     *
     * @param title The notification title
     * @param options Optional message, severity, and duration
     *
     * @example
     * ```ts
     * dialogs.notify("Flight deleted", { severity: "success" });
     * dialogs.notify("Something broke", { severity: "error", duration: 0 });
     * ```
     */
    notify(
        title: string,
        options?: Partial<
            Pick<NotifyDialog, "message" | "severity" | "duration">
        >
    ): void {
        const duration: number = options?.duration ?? 8000;

        this.notification = {
            type: "notify",
            title,
            message: options?.message,
            severity: options?.severity ?? "info",
            duration,
        };

        if (duration > 0) {
            setTimeout((): void => {
                this.notification = null;
            }, duration);
        }
    }

    // =========================================================================
    // Resolver actions
    //
    // These are called by the dialog components (via DialogShell) when the
    // user interacts. Unless you're dabbling in the forbidden (building a custom
    // dialog variant), you shouldn't need to call these directly.
    // =========================================================================

    /**
     * Resolves the active dialog with the given value and closes it.
     *
     * Called by dialog components when the user clicks Confirm, Submit, OK.
     * For custom dialogs: call this with whatever value you want the
     * Promise to resolve to.
     *
     * @param value The value to resolve with
     *
     * @example
     * ```ts
     * // TODO: GET A BETTER EXMAPLE
     * ```
     */
    resolve(value: any): void {
        if (this.#_resolve) this.#_resolve(value);
        this.active = null;
        this.#_resolve = null;
    }

    /**
     * Dismisses the active dialog without a positive answer.
     *
     * Called when the user presses Escape, clicks the backdrop, or hits
     * Cancel. Resolves with the "cancelled" value for each type:
     * - confirm → `false`
     * - prompt / form → `null`
     * - alert → `undefined`
     *
     * @example
     * ```ts
     * // TODO: GET A BETTER EXMAPLE
     * ```
     */
    dismiss(): void {
        if (this.#_resolve) {
            switch (this.active?.type) {
                case "confirm":
                    this.#_resolve(false);
                    break;
                case "prompt":
                case "form":
                    this.#_resolve(null);
                    break;
                default:
                    this.#_resolve(undefined);
                    break;
            }
        }
        this.active = null;
        this.#_resolve = null;
    }

    /**
     * Dismisses the current notification toast immediately.
     */
    dismissNotification(): void {
        this.notification = null;
    }
}

/**
 * Singleton store — import this anywhere in the app.
 *
 * ```ts
 * import { dialogs } from "$lib/stores/dialog.svelte";
 * ```
 */
export const dialogs = new DialogStore();
