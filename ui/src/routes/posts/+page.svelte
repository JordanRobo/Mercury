<script lang="ts">
	import type { Post } from "$lib/types/post";
	import { Button } from "$lib/components/ui/button";
	import { goto } from "$app/navigation";

	export let data: Post[];
</script>

<main>
	{#if data.length > 0}
		{#each data as post}
			<a href="/posts/{post.id}">{post.title}</a>
		{/each}
	{:else}
		<h2>You don't have any posts yet</h2>
		<Button on:click={() => goto("/posts/new")}>New Post</Button>
	{/if}
</main>
