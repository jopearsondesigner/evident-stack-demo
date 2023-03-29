import { initializeEventModelStore, eventModelGrid } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';
import { derived } from 'svelte/store';

export const load: LayoutLoad = async ({ params }) => {
  if (browser) {
    let {state, dispatch} = await initializeEventModelStore(params.id);
    return {
      eventModel: state,
      grid: derived(state, eventModelGrid),
      dispatch
    };
  }
};
