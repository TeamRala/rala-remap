<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import * as Dialog from "$shadcn/dialog";
	import { Input } from "$shadcn/input";
	import { buttonVariants } from "$shadcn/button";
	import AddMappingDialog from "$lib/components/AddMappingDialog.svelte";
	import MappingsList from "$lib/components/MappingsList.svelte";
	import CustomTitlebar from "$lib/components/CustomTitlebar.svelte";

	type Mapping = {
		mouse_button: number;
		keyboard_key: string;
	};

	let mouseMappings: Mapping[] = [];

	onMount(async () => {
		mouseMappings = await invoke("get_mappings");
	});

	async function addMapping(mapping: Mapping) {
		await invoke("add_mapping", { mapping });
		mouseMappings = await invoke("get_mappings");

		console.log(mouseMappings);
	}

	async function deleteMapping(mouseButton: number) {
		await invoke("remove_mapping", { mouseButton });
		mouseMappings = await invoke("get_mappings");
	}
</script>

<div class="flex flex-col h-screen">
	<CustomTitlebar />
	<div class="flex-grow p-8 flex flex-col gap-4 overflow-auto">
		<h1 class="text-2xl font-bold">Mouse to Keyboard key remapping</h1>

		<!-- TEST INPUT -->
		<Input type="text" placeholder="Type something..." />

		<div class="space-y-4">
			<MappingsList {mouseMappings} on:delete={e => deleteMapping(e.detail)} />

			<Dialog.Root>
				<Dialog.Trigger class={buttonVariants({ variant: "default" })}>Add mapping</Dialog.Trigger>
				<AddMappingDialog on:addMapping={e => addMapping(e.detail)} />
			</Dialog.Root>
		</div>
	</div>
</div>
