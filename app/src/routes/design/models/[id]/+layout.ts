import { initializeEventModelStore, eventModelGrid } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';
import { derived } from 'svelte/store';

export const load: LayoutLoad = async ({ params }) => {
  if (browser) {
    let {state, import_json, delete_model} = await initializeEventModelStore(params.id);
    return {
      grid: derived(state, eventModelGrid),
      import_json, delete_model
    };
  }
};
