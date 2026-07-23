<script lang="ts">
    import type { Branch } from "$lib/types/Branch";
    import BranchView from "$lib/components/workspace/BranchView.svelte";
    import BaseWidget from "$lib/components/widgets/BaseWidget.svelte";

    interface Props {
        /** The container's body content */
        branch: Branch;
    }

    let { branch }: Props = $props();
</script>

<div class="split-{branch.direction}">
    {#each branch.children as child, i (i)}
        {#if "Branch" in child}
            <!-- If the child is itself a Branch, call this component again with the child -->
            <BranchView branch={child.Branch} />
        {:else}
            <!-- Otherwise (it's a Leaf), render the Leaf -->
            <BaseWidget leaf={child.Leaf} />
        {/if}
    {/each}
</div>
