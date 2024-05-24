<script lang="ts">
    import "./editor.scss"

    import type { PageData } from './$types';
    import { onMount } from 'svelte';
    import { Editor } from '@tiptap/core';
    import StarterKit from '@tiptap/starter-kit';
    import Typography from '@tiptap/extension-typography';
    import Highlight from '@tiptap/extension-highlight';
    import Button from "$lib/components/ui/button/button.svelte";
    import { enhance } from "$app/forms";

    let element: any;
    let editor: any;

    export let data: PageData;

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
