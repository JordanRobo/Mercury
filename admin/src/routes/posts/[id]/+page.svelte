<script lang="ts">
    import "./editor.scss"

    import type { PageData, ActionData } from './$types';
    import { onMount } from 'svelte';
    import { Editor } from '@tiptap/core';
    import StarterKit from '@tiptap/starter-kit';
    import Typography from '@tiptap/extension-typography';
    import Highlight from '@tiptap/extension-highlight';
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Alert from "$lib/components/ui/alert";
    import { enhance } from "$app/forms";

    let element: any;
    let editor: any;

    export let data: PageData;
    export let form: ActionData;

	let showMessage = false;

	$: if (form?.success !== undefined) {
		showMessage = true;
		setTimeout(() => {
			showMessage = false;
		}, 5000); 
	};

    let htmlContent = '';

    $: if (editor) {
        htmlContent = editor.getHTML();
    }
    
    onMount(() => {
        editor = new Editor({
            element: element,
            extensions: [
                StarterKit,
                Typography,
                Highlight
            ],
            content: `${data.post.content}`,
            onTransaction: () => {
            editor = editor;
            },
        });
    });
</script>

<div class="p-8 space-y-4">
    <div class="p-2" bind:this={element} />
    <form method="POST" action="?/save" use:enhance>
        <input name="id" value={data.post.id} hidden/>
        <textarea name="content" bind:value={htmlContent} hidden/>
        <Button type="submit">Save</Button>
    </form>
</div>
{#if showMessage && form?.success === true}
    <Alert.Root>
        <Alert.Title>Success!</Alert.Title>
        <Alert.Description>
            Your post was saved successfully.
        </Alert.Description>
    </Alert.Root>
{:else if showMessage && form?.success === false}
    <Alert.Root>
        <Alert.Title>Oh no!</Alert.Title>
        <Alert.Description>
            There was an issue with your post, try again in a moment.
        </Alert.Description>
    </Alert.Root>
{/if}