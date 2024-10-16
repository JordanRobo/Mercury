<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { auth } from "$lib/stores";
</script>

<Button on:click={auth.login}>Login</Button>
