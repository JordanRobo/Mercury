<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { auth } from "$lib/stores";
</script>

<h1>Welcome to Mercury</h1>
<p>
    Visit <a href="https://mercury.rs" class="text-teal-600 hover:text-teal-400"
        >mercury.rs</a
    > to read the documentation
</p>

<Button on:click={auth.logout}>Logout</Button>
