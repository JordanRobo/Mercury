<script lang="ts">
	import { auth } from "$lib/stores/auth";
</script>

<div class="flex min-h-dvh items-center justify-center">
	<div class="text-center">
		<h2 class="text-2xl">Hey there {$auth.user?.name}!</h2>
		<h1 class="text-6xl font-bold text-primary">Welcome to Mercury</h1>
		<p class="pt-4">
			Visit <a href="https://mercury.rs" class="text-teal-600 hover:text-teal-400">mercury.rs</a> to read the documentation
		</p>
	</div>
</div>
