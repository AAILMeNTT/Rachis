<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import SectionHeader from "$lib/components/layout/SectionHeader.svelte";

    const { Story } = defineMeta({
        title: "Landing/SectionHeader",
        component: SectionHeader,
        argTypes: {
            title: { control: "text" },
            subtitle: { control: "text" },
        }
    });
</script>

<Story
    name="Welcome Back"
    args={{
        title: "Welcome Back",
        subtitle: "Pick up where you left off",
    }} />

<Story
    name="Favourites"
    args={{
        title: "Favourites",
        subtitle: "Your favourite eggs, all in one basket",
    }} />

<Story
    name="Older Projects"
    args={{ title: "Older Projects", subtitle: "Your previous works" }} />
