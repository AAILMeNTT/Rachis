<script lang="ts">
</script>

<main>
    <ul>
        <li>
            {#if import.meta.env.DEV}
                <a href="./dev">To Developer Panel</a>
            {:else}
                <p>To Developer Panel (Disabled)</p>
            {/if}
        </li>
        <li>
            <a href="./dev/widget-test/editor">To Editor Widget Test</a>
        </li>
    </ul>
</main>
