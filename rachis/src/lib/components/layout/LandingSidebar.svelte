<script lang="ts">
    import "$lib/styles/app.css";
    import { landingStore } from "$lib/stores/landing.svelte";
    import Moon from "@lucide/svelte/icons/moon";
    import Button from "$lib/components/layout/Button.svelte";

    interface Props {
        user?: { name: string };
    }

    let { user }: Props = $props();
</script>

<div
    class="sidebar
    w-75 h-full bg-white flex flex-col flex-nowrap items-start content-stretch justify-between gap-y-3 px-9 py-9">
    <div
        class="top-sidebar
        w-full flex flex-col flex-nowrap items-start content-stretch justify-start gap-y-3">
        <div
            class="greeting
            w-full h-auto flex flex-col flex-nowrap items-start content-stretch justify-start gap-y-2">
            <!-- TODO: Get actual user -->
            <!-- Well... considering this is a purely offline application... does there really need to be a user? I guess there could be some local setting so that the user can put their author name (and perhaps set it per-Flight as well in case they want to change that?), but I don't think there needs to be a whole user object or anything -->
            <p>Hello, {user?.name || "Author"}</p>
            <p>
                {landingStore.total_flights} flights · {landingStore.total_word_count}
                words
            </p>
        </div>
        <div
            class="divider
            w-full h-0.5 bg-wisteria">
        </div>
        <div
            class="sidebar-actions
            w-full h-auto flex flex-col items-center content-stretch justify-start gap-y-3 px-2 py-6">
            <Button text="Open Flight" type="disabled" args={[]} />
            <!-- TODO: Dialogue box here too, not hardcode -->
            <Button
                text="New Flight"
                type="tertiary"
                args={["MLP: Ad Eternum", "/projects/mlp_ad_eternum"]} />
            <Button text="Settings" type="disabled" args={[]} />
            <Button text="Getting Started" type="disabled" args={[]} />
        </div>
    </div>
    <div
        class="footer
        w-full h-auto flex flex-col flex-nowrap items-start content-stretch justify-start gap-y-3 pbs-3">
        <div
            class="divider
            w-full h-0.5 bg-wisteria">
        </div>
        <div
            class="info
            w-full h-auto flex flex-row flex-nowrap items-center content-stretch justify-between px-2 py-1">
            <p>v0.4.0</p>
            <Moon />
        </div>
    </div>
</div>
