import { initializeEventModelStore } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
  if (browser) {
    let {store, dispatch} = initializeEventModelStore(params.id);
    return {
      eventModel: store,
      dispatch
    };
  }
};
