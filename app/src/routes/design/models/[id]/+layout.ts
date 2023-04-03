import { initializeEventModelStore } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
  if (browser) {
    let {state, import_json, delete_model} = await initializeEventModelStore(params.id);
    return {
      grid: state,
      import_json, delete_model
    };
  }
};
