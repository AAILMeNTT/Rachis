<script lang="ts">
    import { dialogs } from "$lib/stores/dialog.svelte";
    
    interface Props {}

    let {}: Props = $props();
</script>

<div class="notification-content">
    <p class="notification-title">{dialogs.notification?.title}</p>
    {#if dialogs.notification?.message}
        <p class="notification-message">
            {dialogs.notification?.message}
        </p>
    {/if}
</div>
<button
    class="notification-close"
    onclick={(): void => dialogs.dismissNotification()}
    aria-label="Dismiss notification">
    ×
</button>
