<script lang="ts">
    import { landingStore } from "$lib/stores/landing.svelte";
</script>

<div class="flex gap-x-4 justify-end">
    <div class="stats-pill">
        <span>{landingStore.total_flights}</span>
        <span>Flights</span>
    </div>
    <div class="stats-pill">
        <!-- TODO: Total Rachis count across all Flights -->
        <span>0</span>
        <span>Rachises</span>
    </div>
    <div class="stats-pill">
        <span>{landingStore.total_word_count.toLocaleString()}</span>
        <span>Words</span>
    </div>
</div>
