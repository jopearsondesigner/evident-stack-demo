import { initialize_decider } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (event) => {
  if (browser) {
    let {session} = await event.parent()
    let {grid, decider} = await initialize_decider(event.params.id, session.user!.id);
    return {grid, decider};
  }
};
