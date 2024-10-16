<script lang="ts">
    import { auth } from "$lib/stores";
    import "../app.css";
    import Login from "$lib/components/login/Login.svelte";
</script>

<main class="container mx-auto">
    {#if !$auth}
        <Login />
    {:else}
        <slot />
    {/if}
</main>
