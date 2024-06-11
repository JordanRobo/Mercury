<script lang="ts">
	import Ellipsis from "lucide-svelte/icons/ellipsis";
	import { Badge } from "$lib/components/ui/badge/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import * as Table from "$lib/components/ui/table";
	import { goto } from "$app/navigation";

    export let posts: {id: string, title: string, published: boolean, author: { name: string }}[];
	export let lower: number;
	export let upper: number;
	export let total: number;
</script>

<main class="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
    <div class="flex items-center">
        <h1 class="text-lg font-semibold md:text-2xl">Posts</h1>
    </div>
<Card.Root>
	<Card.Content>
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>Title</Table.Head>
					<Table.Head>Status</Table.Head>
					<Table.Head>Author</Table.Head>
					<Table.Head>
						<span class="sr-only">Actions</span>
					</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
                {#each posts as post (post.id)}
				<Table.Row>
					<Table.Cell class="font-medium">{post.title}</Table.Cell>
					<Table.Cell>
						<Badge variant="outline">
							{#if post.published}
								Published
							{:else}
								Draft
							{/if}
						</Badge>
					</Table.Cell>
					<Table.Cell>{post.author.name}</Table.Cell>
					<Table.Cell>
						<DropdownMenu.Root>
							<DropdownMenu.Trigger asChild let:builder>
								<Button
									aria-haspopup="true"
									size="icon"
									variant="ghost"
									builders={[builder]}
								>
									<Ellipsis class="h-4 w-4" />
									<span class="sr-only">Toggle menu</span>
								</Button>
							</DropdownMenu.Trigger>
							<DropdownMenu.Content align="end">
								<DropdownMenu.Label>Actions</DropdownMenu.Label>
								<DropdownMenu.Item on:click={() => goto(`/posts/${post.id}`)}>Edit</DropdownMenu.Item>
								<DropdownMenu.Item>Delete</DropdownMenu.Item>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					</Table.Cell>
				</Table.Row>
                {/each}
			</Table.Body>
		</Table.Root>
	</Card.Content>
	<Card.Footer>
		<div class="text-xs text-muted-foreground">
			Showing <strong>{lower}-{upper}</strong> of <strong>{total}</strong> posts
		</div>
	</Card.Footer>
</Card.Root>
</main>