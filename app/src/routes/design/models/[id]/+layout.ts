import { initialize_decider } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
  if (browser) {
    let {grid, decider} = await initialize_decider(params.id);
    return {grid, decider};
  }
};
