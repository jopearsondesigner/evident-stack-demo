import { sveltekit } from '@sveltejs/kit/vite';
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

/** @type {import('vite').UserConfig} */
const config = {
  server: {
    fs: {
      allow: ["./state/client/pkg", "./state/server/pkg"]
    }
  },
  plugins: [sveltekit(), wasm(), topLevelAwait()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
};

export default config;
