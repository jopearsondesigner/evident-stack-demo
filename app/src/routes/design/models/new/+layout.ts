import { initializeEventModelStore } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (_event) => {
  if (browser) {
    let {state, dispatch} = await initializeEventModelStore(null);
    return { eventModel: state, dispatch};
  }
};
