<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { toast } from "svelte-sonner";
	import { register } from "$lib/api/auth";

	let name = "";
	let email = "";
	let password = "";
	let password_verify = "";

	async function handleSubmit() {
		if (password != password_verify) {
			toast.error("Passwords do not match, please try again.");
		} else {
			const success = await register(name, email, password);
			if (!success) {
				toast.error("Error creating user");
			} else {
				toast.success("Successfully created admin account");
			}
		}
	}
</script>

<form on:submit|preventDefault={handleSubmit} class="flex items-center h-dvh">
	<Card.Root class="w-full max-w-sm mx-auto">
		<Card.Header>
			<Card.Title class="text-2xl">Create Admin Account</Card.Title>
			<Card.Description>Enter your details below to create your admin account.</Card.Description>
		</Card.Header>
		<Card.Content class="grid gap-4">
			<div class="grid gap-2">
				<Label for="name">Name</Label>
				<Input id="neame" type="text" bind:value={name} placeholder="Jordan Robinson" required />
			</div>
			<div class="grid gap-2">
				<Label for="email">Email</Label>
				<Input id="email" type="email" bind:value={email} placeholder="jordan@mercury.rs" required />
			</div>
			<div class="grid gap-2">
				<Label for="password">Password</Label>
				<Input id="password" type="password" bind:value={password} required />
			</div>
			<div class="grid gap-2">
				<Label for="password_verify">Confirm Passwiord</Label>
				<Input id="password_verify" type="password" bind:value={password_verify} required />
			</div>
		</Card.Content>
		<Card.Footer>
			<Button type="submit" class="w-full">Sign in</Button>
		</Card.Footer>
	</Card.Root>
</form>
