<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import LandingContent from "$lib/components/layout/LandingContent.svelte";

    const { Story } = defineMeta({
        title: "Landing/Content",
        component: LandingContent,
    });
</script>

<Story name="Content" />
