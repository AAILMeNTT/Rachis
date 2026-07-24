import type { Component } from "svelte";
import type { WidgetType } from "$lib/types/WidgetType";

// All Widget modules found in the `widgets` directory
// TIL Svelte's $lib alias doesn't work reliably in import.meta.glob keys so don't try it at home
const widgetModules: Record<string, { default: any }> = import.meta.glob<{
    default: any;
}>("../components/widgets/*Widget.svelte", { eager: true });

// Ugliest syntax known to man
const path: (widgetType: WidgetType) => string = (
    widgetType: WidgetType
): string => `../components/widgets/${widgetType}Widget.svelte`;

export function resolveWidget(widgetType: WidgetType): Component | null {
    const key: string = path(widgetType);
    // console.log("[widget_registry] glob keys:", Object.keys(widgetModules));
    // console.log("[widget_registry] looking up:", key);
    // console.log("[widget_registry] found:", widgetModules[key]?.default ?? null);
    return widgetModules[key]?.default ?? null;
}
