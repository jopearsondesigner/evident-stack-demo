import { initialize_decider } from '$lib/state/event_model';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (event) => {
  if (browser) {
    let {session} = await event.parent()
    let {decider} = await initialize_decider(undefined, session.user!.id);
    return {create_model: decider.create_model};
  }
};
