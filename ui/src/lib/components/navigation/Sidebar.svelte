<script lang="ts">
	import { auth } from "$lib/stores/auth";
	import { Exit, Gear, QuestionMarkCircled, Sun, Moon, Home, Pencil2, Person, EnvelopeClosed, Image, File } from "svelte-radix";
	import { toggleMode } from "mode-watcher";
	import { Button } from "$lib/components/ui/button";

	function handleLogout() {
		auth.logout();
	}
</script>

<nav class="fixed top-0 left-0 w-full h-full border-r bg-background space-y-8 sm:w-80">
	<div class="flex flex-col h-full">
		<div class="h-20 flex items-center px-8">
			<p class="text-2xl font-medium">ACME</p>
		</div>
		<div class="flex-1 flex flex-col h-full overflow-auto">
			<ul class="px-4 text-sm font-medium flex-1">
				<li>
					<a href="/dashboard" class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
						<Home class="h-5 w-5 opacity-80" />
						Dashboard
					</a>
				</li>
				<li>
					<a href="/posts" class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
						<Pencil2 class="h-5 w-5 opacity-80" />
						Posts
					</a>
				</li>
				<li>
					<a href="/pages" class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
						<File class="h-5 w-5 opacity-80" />
						Pages
					</a>
				</li>
				<li>
					<a href="/media" class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
						<Image class="h-5 w-5 opacity-80" />
						Media
					</a>
				</li>
				<li>
					<a href="/messages" class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
						<EnvelopeClosed class="h-5 w-5 opacity-80" />
						Messages
					</a>
				</li>
				{#if $auth.user?.role == "Admin"}
					<li>
						<a href="/users" class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
							<Person class="h-5 w-5 opacity-80" />
							Users
						</a>
					</li>
				{/if}
			</ul>
			<div>
				<ul class="px-4 pb-4 text-sm font-medium">
					<li>
						<a href="https://mercury.rs/docs/what-is-mercury-cms" target="_blank" class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
							<QuestionMarkCircled class="h-5 w-5 opacity-80" />
							Help
						</a>
					</li>
					<li>
						<a href="/settings" class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
							<Gear class="h-5 w-5 opacity-80" />
							Settings
						</a>
					</li>
					<li>
						<button on:click={handleLogout} class="flex items-center gap-x-2 p-2 rounded-lg duration-150">
							<Exit class="h-5 w-5 opacity-80" />
							Logout
						</button>
					</li>
				</ul>
				<div class="py-4 px-4 border-t">
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-x-4">
							<img src="https://avatars.githubusercontent.com/u/147454240?v=4" alt="Profile Headshot" class="w-12 h-12 rounded-full" />
							<div>
								<span class="block text-sm font-semibold">{$auth.user?.name}</span>
								<a href="/settings/user" class="block mt-px text-xs">View Profile</a>
							</div>
						</div>
						<div>
							<Button on:click={toggleMode} variant="outline" size="icon">
								<Sun class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
								<Moon class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
								<span class="sr-only">Toggle theme</span>
							</Button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</nav>
