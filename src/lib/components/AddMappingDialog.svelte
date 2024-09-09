<script lang="ts">
	import * as Dialog from "$shadcn/dialog";
	import { Button } from "$shadcn/button";
	import { Dialog as DialogPrimitive } from "bits-ui";
	import { Input } from "$shadcn/input";

	interface Props {
		addMapping: (mouseButton: number, keyboardKey: string) => void;
	}

	let { addMapping }: Props = $props();
	let mouseButton: number | null = $state(null);
	let keyboardKey = $state("Press a key");

	function handleMouseDown(event: MouseEvent) {
		event.preventDefault();
		mouseButton = event.button;
	}

	function handleKeyDown(event: KeyboardEvent) {
		event.preventDefault();
		keyboardKey = event.key;
	}

	function handleSubmit() {
		if (mouseButton !== null && keyboardKey) {
			addMapping(mouseButton, keyboardKey);
		}
	}
</script>

<Dialog.Content>
	<Dialog.Header>
		<Dialog.Title>Add New Mapping</Dialog.Title>
		<Dialog.Description>Map a mouse button to a keyboard key.</Dialog.Description>
	</Dialog.Header>

	<div class="grid gap-4 py-4">
		<div class="grid grid-cols-4 items-center gap-4">
			<label for="mouseButton" class="text-right">Mouse Button:</label>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				id="mouseButton"
				class="col-span-3 border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 cursor-pointer"
				onmousedown={handleMouseDown}
			>
				{mouseButton !== null ? `Button ${mouseButton}` : "Click here"}
			</div>
		</div>
		<div class="grid grid-cols-4 items-center gap-4">
			<label for="keyboardKey" class="text-right">Keyboard Key:</label>
			<Input
				id="keyboardKey"
				class="col-span-3"
				value={keyboardKey}
				on:keydown={handleKeyDown}
				readonly
			/>
		</div>
	</div>

	<Dialog.Footer>
		<DialogPrimitive.Close asChild let:builder>
			<Button on:click={handleSubmit} builders={[builder]}>Add Mapping</Button>
		</DialogPrimitive.Close>
	</Dialog.Footer>
</Dialog.Content>
