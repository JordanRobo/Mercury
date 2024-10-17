<script lang="ts">
	import "../app.css";
	import { auth } from "$lib/stores/auth";
	import Login from "$lib/components/login/Login.svelte";
	import { onMount } from "svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	import { ModeWatcher } from "mode-watcher";

	let isLoading = true;

	onMount(() => {
		auth.checkAuth();
		isLoading = false;
	});
</script>

<Toaster richColors position="top-center" />
<ModeWatcher />

<main class="container mx-auto">
	{#if isLoading}
		<p>Loading...</p>
	{:else if !$auth.isAuthenticated}
		<Login />
	{:else}
		<slot />
	{/if}
</main>
