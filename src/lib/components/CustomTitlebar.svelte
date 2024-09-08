<script lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	import { Button } from "$shadcn/button";

	async function minimizeWindow() {
		await appWindow.minimize();
	}

	async function closeWindow() {
		await appWindow.hide();
	}
</script>

<div
	class="flex justify-between items-center bg-primary text-primary-foreground p-2"
	data-tauri-drag-region
>
	<div class="flex-grow pointer-events-none">Rala Remap</div>
	<div class="">
		<Button variant="ghost" size="icon" on:click={minimizeWindow}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="24"
				height="24"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12" /></svg
			>
		</Button>
		<Button variant="ghost" size="icon" on:click={closeWindow}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="24"
				height="24"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg
			>
		</Button>
	</div>
</div>
