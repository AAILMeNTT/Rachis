<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import Section from "$lib/components/layout/Section.svelte";

    const { Story } = defineMeta({
        title: "Landing/Section",
        component: Section,
        argTypes: {
            title: { control: "text" },
            subtitle: { control: "text" },
        },
    });
</script>

<Story
    name="Welcome Back"
    args={{
        title: "Welcome Back",
        subtitle: "Your latest writing, ready for flight",
    }} />

<Story
    name="Favourites"
    args={{
        title: "Favourites",
        subtitle: "Your favourite eggs, all in one basket",
    }} />

<Story
    name="Older Projects"
    args={{
        title: "Older Projects",
        subtitle: "Your previous works, comfortably settled in",
    }} />
