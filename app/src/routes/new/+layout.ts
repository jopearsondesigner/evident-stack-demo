import { initializeEventModelStore } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (_event) => {
  if (browser) {
    let {create_model} = await initializeEventModelStore(undefined);
    return {create_model};
  }
};
