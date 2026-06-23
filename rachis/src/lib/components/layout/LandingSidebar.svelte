<script lang="ts">
    import "$lib/styles/app.css";
    import { landingStore } from "$lib/stores/landing.svelte";
    import Moon from "@lucide/svelte/icons/moon";
    import Button from "$lib/components/layout/Button.svelte";
    import { dialogs } from "$lib/stores/dialog.svelte";

    interface Props {
        /** probably gonna get rid of this asp don't get used to it */
        user?: { name: string };
    }

    let { user }: Props = $props();

    /** Prompts the user to select a Flight to open */
    async function handleOpenFlight(): Promise<void> {
        // If the user has no flights, show an alert and return
        if (landingStore.flights.length === 0) {
            await dialogs.alert(
                "No Flights",
                "Create a new flight to get started!"
            );
            return;
        }

        // TODO: we'll need a new Dialog so that the user can select a path or smth
        // Or maybe it just brings up a list of all the user's Flights and they select it
        // yeah that sounds better
    }

    /** Prompts the user to provide information to create a new Flight */
    async function handleNewFlight(): Promise<void> {
        // Present a form to the user to fill out the details of the new Flight
        const result: Record<string, string> | null = await dialogs.form(
            "New Flight",
            [
                {
                    id: "name",
                    label: "Project Name",
                    type: "text",
                    placeholder: "MLP: Ad Eternum",
                    required: true,
                },
                {
                    id: "path",
                    label: "File Path",
                    type: "text",
                    placeholder: "/projects/mlp_ad_eternum",
                },
            ],
            { submitText: "Let's fly!" }
        );

        // If the user submits the form, create the new Flight and notify the user
        if (result) {
            const name: string = result.name;
            // TODO: more aggressive snake-casing so that the names are valid regardless of platform
            const path: string =
                result.path
                || `/projects/${name.toLowerCase().replace(/\s+/g, "_")}.rachis`;
            await landingStore.add(name, path);
            dialogs.notify(`Created "${name}"`, { severity: "success" });

            // TODO: Open the new Flight immediately
        }
    }
</script>

<div
    class="sidebar
    w-75 h-full bg-white flex flex-col flex-nowrap items-start content-stretch justify-between gap-y-3 px-9 py-9">
    <div
        class="top-sidebar
        w-full h-full flex flex-col flex-nowrap items-start content-stretch justify-start gap-y-3">
        <div
            class="greeting
            w-full h-auto flex flex-col flex-nowrap items-start content-stretch justify-start gap-y-2">
            <!-- TODO: Get actual user -->
            <!-- Well... considering this is a purely offline application... does there really need to be a user? I guess there could be some local setting so that the user can put their author name (and perhaps set it per-Flight as well in case they want to change that?), but I don't think there needs to be a whole user object or anything -->
            <p>Hello, {user?.name || "Author"}</p>
            <p>
                {landingStore.total_flights} flights · {landingStore.total_word_count.toLocaleString()}
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
            <Button
                text="Open Flight"
                type="primary"
                onclick={handleOpenFlight} />
            <Button
                text="New Flight"
                type="tertiary"
                onclick={handleNewFlight} />
            <Button text="Settings" type="disabled" />
            <Button text="Getting Started" type="disabled" />
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
            <p>v0.5.0</p>
            <Moon />
        </div>
    </div>
</div>
