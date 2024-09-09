<script lang="ts">
	import { Button } from "$shadcn/button";

	interface Props {
		mouseMappings: Array<{ mouse_button: number; keyboard_key: string }>;
		deleteMapping: (mouseButton: number) => void;
	}

	let { mouseMappings, deleteMapping }: Props = $props();
</script>

{#if mouseMappings.length > 0}
	<ul class="space-y-2">
		{#each mouseMappings as mapping}
			<li class="flex justify-between items-center bg-secondary p-2 rounded">
				<span>Mouse Button {mapping.mouse_button} → {mapping.keyboard_key}</span>
				<Button variant="destructive" onclick={() => deleteMapping(mapping.mouse_button)}>
					Delete
				</Button>
			</li>
		{/each}
	</ul>
{:else}
	<p class="text-muted-foreground">No mappings added yet.</p>
{/if}
