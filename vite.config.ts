import { sveltekit } from "@sveltejs/kit/vite";
import type { UserConfig } from "vite";
import unocss from "unocss/vite";
import Icons from "unplugin-icons/vite";

const config: UserConfig = {
	plugins: [unocss(), Icons({ compiler: "svelte", scale: 1.2 }), sveltekit()],
};

export default config;
