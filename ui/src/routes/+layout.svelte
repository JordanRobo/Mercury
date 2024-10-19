<script lang="ts">
	import "../app.css";
	import { auth, adminExists } from "$lib/stores/auth";
	import Login from "$lib/components/login/Login.svelte";
	import Register from "$lib/components/register/Register.svelte";
	import { onMount } from "svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	import { ModeWatcher } from "mode-watcher";
	import { adminCheck } from "$lib/api/auth";

	let isLoading = true;

	onMount(() => {
		auth.checkAuth();
		adminCheck();
		isLoading = false;
	});
</script>

<Toaster richColors position="top-center" />
<ModeWatcher />

<main class="container mx-auto">
	{#if isLoading}
		<p>Loading...</p>
	{:else if !$adminExists}
		<Register />
	{:else if !$auth.isAuthenticated}
		<Login />
	{:else}
		<slot />
	{/if}
</main>
