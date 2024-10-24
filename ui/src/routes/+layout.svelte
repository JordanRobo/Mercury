<script lang="ts">
	import "../app.css";
	import { auth } from "$lib/stores/auth";
	import { onMount } from "svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	import { ModeWatcher } from "mode-watcher";
	import Sidebar from "$lib/components/navigation/Sidebar.svelte";
	import Login from "$lib/components/login/Login.svelte";

	onMount(() => {
		auth.checkAuth();
	});
</script>

<Toaster richColors position="top-center" />
<ModeWatcher />

{#if $auth.isLoading}
	<p>Loading...</p>
{:else if !$auth.isAuthenticated}
	<Login />
{:else}
	<main class="container w-full ml-80">
		<Sidebar />
		<slot />
	</main>
{/if}
