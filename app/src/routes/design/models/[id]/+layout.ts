import { initialize_decider } from '$lib/state/event_model';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (event) => {
  if (browser) {
    let { connect } = await import('$lib/state/dexie');
    let { session, supabase } = await event.parent()
    let syncing = await connect(supabase);

    if (session) {
      // TODO: check Dexie for model
      //   if model isn't in Dexie, check Supabase
      //     if model exists in Supabase, render and rely on initial Dexie sync to do its thing
      //     else if model doesn't exist in Supabase, throw 404
      let { grid, decider } = await initialize_decider(event.params.id, session.user.id);
      return { grid, decider, syncing };
    }
    return {};
  }
};
