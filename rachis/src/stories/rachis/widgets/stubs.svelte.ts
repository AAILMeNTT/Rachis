import { WidgetType } from "$lib/types/WidgetType";
import type { TreeNode } from "$lib/types/TreeNode";
import type { Branch } from "$lib/types/Branch";
import type { Leaf } from "$lib/types/Leaf";

/** A minimal Leaf driven by widget_type */
export function stubLeaf(
    overrides: Partial<
        Pick<Leaf, "id" | "widget_type" | "widget_instance_id">
    > = {}
): Leaf {
    return {
        id: "00000000-0000-0000-0000-000000000001",
        widget_type: WidgetType.Picker,
        widget_instance_id: null,
        ...overrides,
    };
}

/** A minimal Branch */
export function stubBranch(
    overrides: Partial<
        Pick<Branch, "id" | "direction" | "children" | "ratios">
    > = {}
): Branch {
    return {
        id: "10000000-0000-0000-0000-000000000001",
        direction: "Horizontal",
        children: [],
        ratios: [],
        ...overrides,
    };
}

export function toTreeNode(node: Branch | Leaf): TreeNode {
    if ("direction" in node) {
        return { Branch: node as Branch };
    } else {
        return { Leaf: node as Leaf };
    }
}
