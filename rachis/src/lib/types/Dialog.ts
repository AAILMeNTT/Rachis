// ============================================================================
// Dialog Types — defines every kind of dialog the system supports
// ============================================================================

// ——— Base ———

export interface BaseDialog {
    type: string;
    title: string;
    message?: string;
}

// ——— Confirm ———

export interface ConfirmDialog extends BaseDialog {
    type: "confirm";
    message: string;
    confirmText?: string;
    cancelText?: string;
    severity?: "default" | "destructive";
}

// ——— Prompt ———

export interface PromptDialog extends BaseDialog {
    type: "prompt";
    label: string;
    placeholder?: string;
    defaultValue?: string;
    submitText?: string;
    cancelText?: string;
}

// ——— Form ———

export interface FormOption {
    value: string;
    label: string;
}

export interface FormField {
    id: string;
    label: string;
    type: "info" | "text" | "select" | "radio";
    placeholder?: string;
    defaultValue?: string;
    required?: boolean;
    options?: FormOption[];
    layout?: "vertical" | "horizontal";
}

export interface FormDialog extends BaseDialog {
    type: "form";
    fields: FormField[];
    submitText?: string;
    cancelText?: string;
}

// ——— Alert ———

export interface AlertDialog extends BaseDialog {
    type: "alert";
    message: string;
    okText?: string;
}

// ——— Notify ———

export interface NotifyDialog extends BaseDialog {
    type: "notify";
    message?: string;
    severity: "info" | "success" | "error";
    duration?: number;
}

/**
 * Union of all blocking dialog types.
 * Must be a `type` — interfaces can't form union types.
 */
export type Dialog = ConfirmDialog | PromptDialog | AlertDialog | FormDialog;
