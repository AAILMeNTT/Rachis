<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Rachis } from "$lib/types/Rachis";
    import { RachisType } from "$lib/types/RachisType";
    import RachisSearch from "./RachisSearch.svelte";

    let rachis_title: string = $state("");
    let rachis_type: RachisType = $state(RachisType.DEFAULT);
    let rachis_content: string = $state("");
    let rachis_path: string = $state("");

    let rachis_id: string = $state("");

    let errorMessage: string = $state("");

    async function createRachis(): Promise<void> {
        errorMessage = "";
        if (!rachis_title.trim()) {
            errorMessage = "Rachis name cannot be empty";
            return;
        }
        try {
            const result: Rachis = await invoke<Rachis>("create_rachis", {
                title: rachis_title,
                type: rachis_type,
                content: rachis_content,
                path: rachis_path,
            });
            if (result) {
                rachis_title = "";
                rachis_type = RachisType.DEFAULT;
                rachis_content = "";
                rachis_path = "";
            }
        } catch (error) {
            errorMessage = String(error);
        }
    }

    async function deleteRachis(): Promise<void> {
        errorMessage = "";
        try {
            await invoke("delete_rachis", { id: rachis_id });
            rachis_id = "";
        } catch (error) {
            errorMessage = String(error);
        }
    }
</script>

<main>
    <h3>Rachis Data (In-Memory)</h3>
    <div>
        <p>Rachis name:</p>
        <input type="text" bind:value={rachis_title} />
    </div>
    <div>
        <p>Type:</p>
        <select name="rachisType" id="rachisType" bind:value={rachis_type}>
            <option value={RachisType.ACT}>Act</option>
            <option value={RachisType.ARC}>Arc</option>
            <option value={RachisType.CHARACTER}>Character</option>
            <option value={RachisType.DEFAULT}>Default</option>
            <option value={RachisType.EVENT}>Event</option>
            <option value={RachisType.ITEM}>Item</option>
            <option value={RachisType.LOCATION}>Location</option>
            <option value={RachisType.NOTE}>Note</option>
            <option value={RachisType.SCENE}>Scene</option>
        </select>
    </div>
    <div>
        <p>Content:</p>
        <textarea
            name="rachisContent"
            id="rachisContent"
            bind:value={rachis_content}>
        </textarea>
    </div>
    <div>
        <p>Path:</p>
        <input type="text" bind:value={rachis_path} />
    </div>
    <h4>Rachis (In-Memory)</h4>
    <div>
        <div>
            <pre>{JSON.stringify(
                    {
                        rachis_title,
                        rachis_type,
                        rachis_content,
                        rachis_path,
                    },
                    null,
                    4
                )}</pre>
        </div>
    </div>
    <div>
        <button onclick={createRachis}>Create New Rachis</button>
    </div>
    <RachisSearch />
    <div>
        <p>Rachis ID (for deletion):</p>
        <input type="text" bind:value={rachis_id} />
        <button onclick={deleteRachis}>Delete</button>
    </div>
    <p>{errorMessage}</p>
</main>
