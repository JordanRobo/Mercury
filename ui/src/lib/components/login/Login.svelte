<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { login } from "$lib/api/auth";

	let email = "";
	let password = "";
	let isLoading = false;

	async function handleSubmit() {
		if (!email || !password) {
			return;
		}

		isLoading = true;
		try {
			await login(email, password);
		} finally {
			isLoading = false;
		}
	}
</script>

<form on:submit|preventDefault={handleSubmit} class="flex items-center h-dvh">
	<Card.Root class="w-full max-w-sm mx-auto">
		<Card.Header class="text-center">
			<Card.Title class="text-2xl">Login</Card.Title>
			<Card.Description>Enter your email below to login to your account.</Card.Description>
		</Card.Header>
		<Card.Content class="grid gap-4">
			<div class="grid gap-2">
				<Label for="email">Email</Label>
				<Input id="email" type="email" bind:value={email} placeholder="jordan@mercury.rs" required disabled={isLoading} />
			</div>
			<div class="grid gap-2">
				<Label for="password">Password</Label>
				<Input id="password" type="password" bind:value={password} required disabled={isLoading} />
			</div>
		</Card.Content>
		<Card.Footer>
			<Button type="submit" class="w-full" disabled={isLoading}>
				{#if isLoading}
					Signing in...
				{:else}
					Sign in
				{/if}
			</Button>
		</Card.Footer>
	</Card.Root>
</form>
