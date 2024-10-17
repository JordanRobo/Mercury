<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { auth } from "$lib/stores/auth";

	function handleLogout() {
		auth.logout();
	}
</script>

<h1>Welcome to Mercury</h1>
<p>
	Visit <a href="https://mercury.rs" class="text-teal-600 hover:text-teal-400">mercury.rs</a> to read the documentation
</p>

{#if $auth.isAuthenticated}
	<button on:click={handleLogout} class="p-2 bg-red-500 text-white rounded">Logout</button>
{/if}
