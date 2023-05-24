import { initialize_decider } from '$lib/state/event_model';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';
import { error } from '@sveltejs/kit';
import { model_by_id as remote_model_by_id } from '$lib/supabase/database';

export const load: LayoutLoad = async (event) => {
  let { session, supabase } = await event.parent()

  const model_id = event.params.id;

  if (session) {
    if (browser) {
      const { model_by_id } = await import('$lib/state/dexie');
      const local_model = await model_by_id(session.user.id, model_id);
      if (local_model) {
        let { grid, decider } = await initialize_decider(model_id, session.user.id);
        return { grid, decider, session };
      } else {
        const remote_model = await remote_model_by_id(supabase, model_id);
        if (remote_model) {
          let { grid, decider } = await initialize_decider(model_id, session.user.id);
          return { grid, decider, session };
        }
      }
    } else {
      return {}
    }
  }
  throw error(404, { message: "not found" });
};
