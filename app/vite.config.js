import { sveltekit } from '@sveltejs/kit/vite';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasm from 'vite-plugin-wasm';

/** @type {import('vite').UserConfig} */
const config = {
  server: {
    fs: {
      allow: ["./state/pkg"]
    }
  },
  plugins: [sveltekit(), wasm(), topLevelAwait()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  },
  optimizeDeps: {
    exclude: [
      'codemirror',
      '@codemirror/language-javascript',
      '@codemirror/language-javascript',
      'svelte-codemirror-editor',
      '@codemirror/lang-javascript',
      '@codemirror/theme-one-dark'
    ]
  }
};

export default config;
