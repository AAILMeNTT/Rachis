<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import Workspace from "$lib/components/workspace/Workspace.svelte";
    import { WidgetType } from "$lib/types/WidgetType";
    import { stubLeaf, stubBranch, toTreeNode } from "./stubs.svelte";

    const { Story } = defineMeta({
        title: "Widgets/Workspace",
        component: Workspace,
        args: {
            root: toTreeNode(stubLeaf()),
        },
        argTypes: {
            root: { control: "object" },
        },
    });
</script>

<Story
    name="WorkspaceEditorLeafChild"
    args={{ root: toTreeNode(stubLeaf({ widget_type: WidgetType.Editor })) }} />

<!-- Commented until I actually add it in but it will be added™️ -->
<!-- <Story
    name="WorkspacePickerLeafChild"
    args={{ root: stubLeaf({ widget_type: WidgetType.Picker }) }} /> -->

<Story
    name="WorkspaceBranchChild"
    args={{
        root: toTreeNode(
            stubBranch({
                children: [
                    toTreeNode(
                        stubLeaf({
                            id: "00000000-0000-0000-0000-000000000001",
                            widget_type: WidgetType.Editor,
                        })
                    ),
                    toTreeNode(
                        stubLeaf({
                            id: "00000000-0000-0000-0000-000000000002",
                            widget_type: WidgetType.Picker,
                        })
                    ),
                    toTreeNode(
                        stubLeaf({
                            id: "00000000-0000-0000-0000-000000000003",
                            widget_type: WidgetType.Picker,
                        })
                    ),
                ],
            })
        ),
    }} />
