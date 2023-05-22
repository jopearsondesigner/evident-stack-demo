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
  worker: {
    format: 'iife', // the default, but making explicit here for wide browser support (esp. Firefox)
    plugins: [sveltekit(), topLevelAwait()],
    rollupOptions: {
      output: {
        // Required to support the 'iife' output format
        inlineDynamicImports: true
      }
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
