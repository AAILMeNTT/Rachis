<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import LandingSidebar from "$lib/components/layout/LandingSidebar.svelte";

    const { Story } = defineMeta({
        title: "Landing/Sidebar",
        component: LandingSidebar,
    });
</script>

<Story name="Sidebar" />
