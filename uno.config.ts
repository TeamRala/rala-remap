import extractorSvelte from "@unocss/extractor-svelte";
import {
	defineConfig,
	presetIcons,
	presetUno,
	presetWebFonts,
	transformerDirectives,
	transformerVariantGroup,
} from "unocss";
import { customPreset, shadcnPreset } from "./presets";
import { fontFamily } from "@unocss/preset-mini/theme";
// https://unocss.dev
export default defineConfig({
	extractors: [extractorSvelte()],
	configDeps: ["./presets/my-preset.ts", "./presets/shadcn-preset.ts"],
	content: {
		filesystem: ["./node_modules/bits-ui/dist/**/*.{html,js,svelte,ts}"],
		pipeline: {
			include: [/\.(vue|svelte|[jt]sx|mdx?|astro|elm|php|phtml|html|ts)($|\?)/],
		},
	},
	theme: {
		fontFamily: {
			sans: ['"Inter"', fontFamily.sans],
		},
	},
	rules: [],
	shortcuts: [
		{
			"btn-primary": "bg-indigo-6 text-white px py-2.5 rd-md fw-medium",
		},
		[
			/^teeny-scrollbar-(w|h)-(\d+)$/,
			([, ax, dg]) => `
      scrollbar-f-thin-rgba(229,229,229,0.4)_rgba(229,229,229,0.04)
      scrollbar:${ax}-${dg}
      scrollbar-track:(rd-2.5 bg-neutral-2/4)
      scrollbar-thumb:(rd-2.5 bg-neutral-2/40)
      `,
		],
		[/^scroll-th-(.+)$/, ([, v]) => `scrollbar-thumb:${v}`],
	],
	variants: [],

	preflights: [
		{
			getCSS: () => `
      body {
        overflow-x: hidden;
      }
      `,
		},
	],

	presets: [
		customPreset,
		shadcnPreset,
		presetUno(),
		presetIcons({ scale: 1.2 }),
		presetWebFonts({
			fonts: {
				inter: "Inter:400;500;600;700",
			},
		}),
	],
	transformers: [transformerDirectives(), transformerVariantGroup()],
});
