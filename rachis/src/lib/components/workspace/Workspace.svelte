<script lang="ts">
    import type { Tree } from "$lib/types/Tree";
    import WidgetShell from "$lib/components/widgets/WidgetShell.svelte";
    import BranchView from "$lib/components/workspace/BranchView.svelte";

    let { root }: Tree = $props();
</script>

<div>
    {#if "Branch" in root}
        <!-- If the child is itself a Branch, create a Branch component-->
        <BranchView branch={root.Branch} />
    {:else}
        <!-- i'll give you three guesses -->
        <WidgetShell leaf={root.Leaf} />
    {/if}
</div>
