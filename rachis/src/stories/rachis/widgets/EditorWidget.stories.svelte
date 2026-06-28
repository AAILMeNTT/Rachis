<script module>
    import { defineMeta } from "@storybook/addon-svelte-csf";
    import EditorWidget from "$lib/components/widgets/EditorWidget.svelte";

    const { Story } = defineMeta({
        title: "Widgets/Editor",
        component: EditorWidget,
        args: {
            title: "Editor Widget",
            isLoading: false,
            error: null,
            content: "",
        },
        argTypes: {
            title: { control: "text" },
            isLoading: { control: "boolean" },
            error: { control: "text" },
            content: { control: "text" },
        },
    });
</script>

<!-- ====================================================================== -->
<!-- Content States                                                         -->
<!-- ====================================================================== -->

<Story
    name="Empty"
    args={{
        title: "Untitled",
        content: "",
    }} />

<Story
    name="WithContent"
    args={{
        title: "Chapter 1",
        content: `# Chapter 1: The Long-Awaited Meeting

The rain hadn't stopped for three days. Twilight Sparkle watched it streak down
the library windows, each droplet a tiny lens distorting the world outside into
something softer, stranger.

"The elements are reacting," Spike said, clutching a scroll.

She turned. "Again?"

"Worse this time."`,
    }} />

<Story
    name="MinimalNoTitle"
    args={{
        title: undefined,
        content: "Just a single paragraph. No title, no frills.",
    }} />

<!-- ====================================================================== -->
<!-- Loading States                                                          -->
<!-- ====================================================================== -->

<Story
    name="Loading"
    args={{
        title: undefined,
        content: "",
        isLoading: true,
    }} />

<Story
    name="LoadingWithTitle"
    args={{
        title: "Chapter 1",
        content: "",
        isLoading: true,
    }} />

<Story
    name="Saving"
    args={{
        title: "Chapter 1",
        content: `# Chapter 1\n\nThe rain hadn't stopped...`,
        isLoading: true,
    }} />

<!-- ====================================================================== -->
<!-- Error States                                                            -->
<!-- ====================================================================== -->

<Story
    name="ErrorNoContent"
    args={{
        title: undefined,
        content: "",
        isLoading: false,
        error: "Failed to load document. The file may have been moved or deleted.",
    }} />

<Story
    name="ErrorWithContent"
    args={{
        title: "Chapter 1",
        content: `# Chapter 1\n\nThe rain hadn't stopped...`,
        isLoading: false,
        error: "Autosave failed. Your latest changes may not be persisted.",
    }} />

<Story
    name="ErrorLoading"
    args={{
        title: undefined,
        content: "",
        isLoading: true,
        error: "Connection lost. Retrying...",
    }} />

<!-- ====================================================================== -->
<!-- Edge Cases                                                              -->
<!-- ====================================================================== -->

<Story
    name="VeryLongContent"
    args={{
        title: "The Neverending Story",
        content: Array.from(
            { length: 50 },
            (_, i) =>
                `## Section ${i + 1}\n\nThis is paragraph ${i + 1} of a very long document `
                + `designed to test scrolling behaviour and performance within the `
                + `editor widget. Each section contains enough text to ensure the `
                + `content overflows the visible area.\n\n`
        ).join("\n"),
    }} />

<Story
    name="SpecialCharacters"
    args={{
        title: "Edge Cases",
        content: `# Special Characters & Edge Cases

## Tags
c!Twilight Sparkle! and l!Ponyville! and e!Summer Sun Celebration!

## HTML
<div class="test">This is literal HTML in the content</div>

## BBCode
[b]Bold[/b] and [i]italic[/i] and [url=https://example.com]link[/url]

## Unicode
中文 español عربى 日本語 हिन्दी

## Emoji
🦄✨📝🎉

## Very long unbroken string
Supercalifragilisticexpialidociousantidisestablishmentmentarianism`,
    }} />
