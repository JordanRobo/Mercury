<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { auth } from "$lib/stores/auth";

	function handleLogout() {
		auth.logout();
	}
</script>

<div class="flex items-center h-dvh">
	<div class="w-full max-w-sm mx-auto space-y-2 text-center">
		<h1 class="text-4xl">Welcome to Mercury</h1>
		<h2 class="text-xl">Your email: {$auth.user?.email}</h2>
		<p>
			Visit <a href="https://mercury.rs" class="text-teal-600 hover:text-teal-400">mercury.rs</a> to read the documentation
		</p>

		{#if $auth.isAuthenticated}
			<Button on:click={handleLogout}>Logout</Button>
		{/if}
	</div>
</div>
