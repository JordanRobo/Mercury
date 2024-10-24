<script lang="ts">
	import "../wysiwyg/Editor.css";
	import EditorJS from "@editorjs/editorjs";
	import Header from "@editorjs/header";
	import Paragraph from "@editorjs/paragraph";
	import { onMount } from "svelte";
	import { random_quote } from "$lib/api/placeholder-quote";
	import { Button } from "$lib/components/ui/button";

	let editor: EditorJS;
	let element: HTMLElement;

	const initialData = {
		blocks: [
			{
				type: "header",
				data: {
					text: "Post Title",
					level: 1,
				},
			},
		],
	};

	onMount(() => {
		editor = new EditorJS({
			holder: element,
			tools: {
				header: {
					class: Header,
					config: {
						levels: [1, 2, 3],
						defaultLevel: 1,
					},
				},
				paragraph: {
					class: Paragraph,
					inlineToolbar: true,
					config: {
						placeholder: random_quote(),
					},
				},
			},
			data: initialData,
			autofocus: true,
		});

		return () => {
			if (editor) {
				editor.destroy();
			}
		};
	});

	async function saveContent() {
		if (editor) {
			try {
				const outputData = await editor.save();
				console.log("Saved data:", outputData);
			} catch (e) {
				console.error("Saving error:", e);
			}
		}
	}
</script>

<div class="flex justify-end gap-2">
	<Button on:click={() => saveContent()} class="w-32" variant="outline">Undo Changes</Button>
	<Button on:click={() => saveContent()} class="w-32">Save</Button>
</div>
<div bind:this={element} class="max-w-full prose prose-lg dark:prose-invert" />
