<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { basicSetup } from "codemirror";
    import { EditorView } from "@codemirror/view";
    import { markdown } from "@codemirror/lang-markdown";
    import { sessionStore as ss } from "$lib/stores/session.svelte";

    let editorElement: HTMLDivElement;
    let view: EditorView;

    let rachisId: string = $state<string>("");

    // Now that we have a way to search and load Rachises, we can use the session store to
    // actually... you know... load the data

    async function loadRachis(): Promise<void> {
        ss.loadRachis(rachisId);
        rachisId = "";

        view?.dispatch({
            changes: [
                {
                    from: 0,
                    to: view.state.doc.length,
                    insert: ss.current_rachis?.content ?? "",
                },
            ],
        });
    }

    onMount((): void => {
        view = new EditorView({
            doc: "Your story begins...",
            extensions: [basicSetup, markdown()],
            parent: editorElement,
        });
    });

    onDestroy((): void => {
        view?.destroy();
    });
</script>

<main>
    <div>
        <p>Load Rachis (by ID):</p>
        <input type="text" bind:value={rachisId} />
        <button onclick={(): Promise<void> => loadRachis()}>Load</button>
        <button onclick={(): Promise<void> => ss.unloadRachis()}>Unload</button>
    </div>
    <div>
        <pre>Loaded Rachis: {JSON.stringify(ss.current_rachis, null, 4)}</pre>
    </div>
    <div bind:this={editorElement}></div>
</main>
