import { initializeEventModelStore } from '$lib/state';
import { browser } from '$app/environment';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
  if (browser) {
    let {store, dispatch} = initializeEventModelStore(params.id);
    return {
      eventModel: store,
      dispatch
    };
  }
}
