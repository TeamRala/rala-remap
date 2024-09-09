<script lang="ts">
	import * as Dialog from "$shadcn/dialog";
	import AddMappingDialog from "$lib/components/AddMappingDialog.svelte";
	import CustomTitlebar from "$lib/components/CustomTitlebar.svelte";
	import MappingsList from "$lib/components/MappingsList.svelte";
	import { Input } from "$shadcn/input";
	import { buttonVariants } from "$shadcn/button";
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	type Mapping = {
		mouse_button: number;
		keyboard_key: string;
	};

	let mouseMappings: Mapping[] = $state([]);

	onMount(async () => (mouseMappings = await invoke("get_mappings")));

	async function addMapping(mouse_button: number, keyboard_key: string) {
		await invoke("add_mapping", { mapping: { mouse_button, keyboard_key } });
		mouseMappings = await invoke("get_mappings");
	}

	async function deleteMapping(mouseButton: number) {
		await invoke("remove_mapping", { mouseButton });
		mouseMappings = await invoke("get_mappings");
	}
</script>

<div class="flex flex-col h-screen relative">
	<CustomTitlebar />
	<div class="flex-grow p-8 flex flex-col gap-4 overflow-auto">
		<h1 class="text-2xl font-bold">Mouse button to Keyboard key remapping</h1>

		<!-- TEST INPUT -->
		<Input type="text" placeholder="Type something..." />

		<div class="space-y-4">
			<MappingsList {mouseMappings} {deleteMapping} />

			<Dialog.Root>
				<Dialog.Trigger class={buttonVariants({ variant: "default" })}>Add mapping</Dialog.Trigger>
				<AddMappingDialog {addMapping} />
			</Dialog.Root>
		</div>
	</div>
</div>
