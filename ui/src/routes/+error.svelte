<script lang="ts">
	import { page } from "$app/stores";
	import { browser } from "$app/environment";
	import { Button } from "$lib/components/ui/button";

	function nav_back() {
		if (browser) window.history.back();
	}
</script>

<div class="flex items-center h-dvh justify-center">
	<div class="space-y-2 text-center">
		<h1 class="text-8xl font-bold text-primary">{$page.error?.message}</h1>
		<h1 class="text-4xl">Whoops, that dosen't seem right...</h1>
		<h2 class="text-xl">Why not try go pack to the last page?</h2>
		<Button on:click={nav_back}>Go Back</Button>
	</div>
</div>
