<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { toast } from "svelte-sonner";
	import { login } from "$lib/api/auth";

	let email = "";
	let password = "";

	async function handleSubmit() {
		const success = await login(email, password);
		if (!success) {
			toast.error("Login failed. Please check your credentials.");
		}
	}
</script>

<form on:submit|preventDefault={handleSubmit} class="flex items-center h-dvh">
	<Card.Root class="w-full max-w-sm mx-auto">
		<Card.Header>
			<Card.Title class="text-2xl">Login</Card.Title>
			<Card.Description>Enter your email below to login to your account.</Card.Description>
		</Card.Header>
		<Card.Content class="grid gap-4">
			<div class="grid gap-2">
				<Label for="email">Email</Label>
				<Input id="email" type="email" bind:value={email} placeholder="m@example.com" required />
			</div>
			<div class="grid gap-2">
				<Label for="password">Password</Label>
				<Input id="password" type="password" bind:value={password} required />
			</div>
		</Card.Content>
		<Card.Footer>
			<Button type="submit" class="w-full">Sign in</Button>
		</Card.Footer>
	</Card.Root>
</form>
