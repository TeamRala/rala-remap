<script lang="ts">
	import { createEventDispatcher } from "svelte";
	import { Button } from "$shadcn/button";

	export let mouseMappings: Array<{ mouse_button: number; keyboard_key: string }>;

	const dispatch = createEventDispatcher();

	function handleDelete(mouseButton: number) {
		dispatch("delete", mouseButton);
	}
</script>

{#if mouseMappings.length > 0}
	<ul class="space-y-2">
		{#each mouseMappings as mapping}
			<li class="flex justify-between items-center bg-secondary p-2 rounded">
				<span>Mouse Button {mapping.mouse_button} → {mapping.keyboard_key}</span>
				<Button variant="destructive" on:click={() => handleDelete(mapping.mouse_button)}
					>Delete</Button
				>
			</li>
		{/each}
	</ul>
{:else}
	<p class="text-muted-foreground">No mappings added yet.</p>
{/if}
