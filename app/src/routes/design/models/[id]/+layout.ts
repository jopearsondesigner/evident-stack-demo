import { initialize_decider } from '$lib/state/event_model';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (event) => {
  if (browser) {
    let { session, supabase } = await event.parent()
    if (session) {
      let { grid, decider } = await initialize_decider(event.params.id, session.user.id, supabase);
      return { grid, decider };
    }
    return {};
  }
};
