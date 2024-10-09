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
    let titleElement: any;
    let titleEditor: any;

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
    let titleContent = '';

    $: if (editor) {
        htmlContent = editor.getHTML();
    }

    $: if (titleEditor) {
        titleContent = titleEditor.getHTML();
    }
    
    onMount(() => {
        editor = new Editor({
            element: element,
            editorProps: {
                attributes: {
                    class: 'focus:outline-primary/20 px-2',
                },
            },
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

        titleEditor = new Editor({
            element: titleElement,
            extensions: [
                StarterKit.configure({
                    heading: {
                        levels: [1],
                    },
                }),
            ],
            editorProps: {
                attributes: {
                    class: 'focus:outline-primary/20 p-2',
                },
            },
            content: `<h1>${data.post.title}</h1>`,
            onTransaction: () => {
                titleEditor = titleEditor;
            },
        });
    });
</script>

<div class="p-8 space-y-4">
    <div bind:this={titleElement} />
    <div bind:this={element} />
    <form method="POST" action="?/save" use:enhance>
        <input name="id" value={data.post.id} hidden/>
        <textarea name="title" bind:value={titleContent} hidden/>
        <textarea name="content" bind:value={htmlContent} hidden/>
        <Button type="submit">Save</Button>
    </form>
</div>

{#if showMessage && form?.success === true}
    <div class="absolute bottom-4 right-4 w-72">
        <Alert.Root>
            <Alert.Title><span class="text-green-400">Success!</span></Alert.Title>
            <Alert.Description>
                Your post was saved successfully.
            </Alert.Description>
        </Alert.Root>
    </div>
{:else if showMessage && form?.success === false}
    <div class="absolute bottom-4 right-4 w-72">
        <Alert.Root>
            <Alert.Title><span class="text-red-400">Oh no!</span></Alert.Title>
            <Alert.Description>
                There was an issue with your post, try again in a moment.
            </Alert.Description>
        </Alert.Root>
    </div>
{/if}