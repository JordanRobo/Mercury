<script lang="ts">
	import '../app.css';

    import { Button } from "$lib/components/ui/button/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
	import { ModeWatcher } from "mode-watcher";
	import { toggleMode } from "mode-watcher";
	import { ChatBubble, Home, MagnifyingGlass, MixerVertical, Moon, Pencil2, Person, Sun, Stack } from 'svelte-radix';
	import type { LayoutData } from './$types';

	export let data: LayoutData;

	function activePath(path: string, url: string) {
		if (path === "/") {
			return url === "/";
		} else {
			return url.startsWith(path);
		}
	};
</script>

<ModeWatcher />

<div class="grid min-h-screen w-full md:grid-cols-[220px_1fr] lg:grid-cols-[280px_1fr]">
	<div class="hidden border-r bg-muted/40 md:block">
		<div class="flex h-full max-h-screen flex-col gap-2">
			<div class="flex h-14 items-center border-b px-4 lg:h-[60px] lg:px-6">
				<a href="/" class="flex items-center gap-2 font-semibold">
					<Stack class="h-6 w-6" />
					<span class="">Acme Inc</span>
				</a>
			</div>
			<div class="flex-1">
				<nav class="grid items-start px-2 text-sm font-medium lg:px-4">
					<a href="/" class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary" class:text-primary={activePath("/", data.url)} class:bg-muted={activePath("/", data.url)}>
						<Home class="h-4 w-4" />
						Dashboard
					</a>
					<a href="/messages" class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary" class:text-primary={activePath("/messages", data.url)} class:bg-muted={activePath("/messages", data.url)}>
						<ChatBubble class="h-4 w-4" />
						Messages
					</a>
					<a href="/posts" class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary" class:text-primary={activePath("/posts", data.url)} class:bg-muted={activePath("/posts", data.url)}>
						<Pencil2 class="h-4 w-4" />
						Posts
					</a>
					<a href="/settings" class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary" class:text-primary={activePath("/settings", data.url)} class:bg-muted={activePath("/settings", data.url)}>
						<MixerVertical class="h-4 w-4" />
						Site Settings
					</a>
				</nav>
			</div>
		</div>
	</div>
	<div class="flex flex-col">
		<header class="flex h-14 items-center gap-4 border-b bg-muted/40 px-4 lg:h-[60px] lg:px-6">
			<div class="w-full flex-1">
				<form>
					<div class="relative">
						<MagnifyingGlass class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
						<Input disabled type="search" placeholder="Search website..." class="w-full appearance-none bg-background pl-8 shadow-none md:w-2/3 lg:w-1/3" />
					</div>
				</form>
			</div>
			<DropdownMenu.Root>
				<DropdownMenu.Trigger asChild let:builder>
					<Button builders={[builder]} variant="secondary" size="icon" class="rounded-full" >
						<Person class="h-5 w-5" />
						<span class="sr-only">Toggle user menu</span>
					</Button>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end">
					<DropdownMenu.Label>My Account</DropdownMenu.Label>
					<DropdownMenu.Separator />
					<DropdownMenu.Item>Settings</DropdownMenu.Item>
					<DropdownMenu.Item>Support</DropdownMenu.Item>
					<DropdownMenu.Separator />
					<DropdownMenu.Item>Logout</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
			<Button on:click={toggleMode} variant="secondary" size="icon" class="rounded-full" >
				<Sun class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
				<Moon class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
				<span class="sr-only">Toggle theme</span>
			</Button>
		</header>
		<slot />
	</div>
</div>