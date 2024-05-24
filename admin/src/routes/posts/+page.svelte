<script lang="ts">
    import type { PageData } from './$types';
    import { NoPosts, Posts } from '$lib';
	import { writable } from 'svelte/store';
    
    export let data: PageData;
    const lower = writable(0);
    const upper = writable(0);
    const total = writable(0);

    $: {
        lower.set((data.posts.length - $upper));
        upper.set(data.posts.length);
        total.set(data.posts.length);
    }
</script>

{#if data.posts.length === 0}
    <NoPosts />
{:else}
    <Posts posts={data.posts} lower={$lower} upper={$upper} total={$total}/>
{/if}