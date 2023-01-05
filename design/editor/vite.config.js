import { sveltekit } from '@sveltejs/kit/vite';
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm-esm";

/** @type {import('vite').UserConfig} */
const config = {
  server: {
    fs: {
      allow: ["./state/pkg"]
    }
  },
	plugins: [sveltekit(), topLevelAwait(), wasm(["editor-state"])],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
};

export default config;
