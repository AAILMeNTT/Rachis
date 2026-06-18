<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import Landing from "$lib/components/layout/Landing.svelte";

    const { Story } = defineMeta({
        title: "Landing/Landing Page",
        component: Landing,
    });
</script>

<Story name="Landing Page" />
