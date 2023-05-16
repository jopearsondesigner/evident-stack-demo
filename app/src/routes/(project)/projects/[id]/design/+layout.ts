import { initialize_decider } from '$lib/state/event_model';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';
import { error } from '@sveltejs/kit';
import { model_by_id } from '$lib/state/dexie';
import type { SupabaseClient } from '@supabase/supabase-js';
import type { Database } from '$lib/supabase/database.types';

const remote_model_by_id = async (supabase: SupabaseClient<Database>, id: string) => {
  let result = await supabase.from("models").select().eq('id', id).limit(1);
  return result.data?.at(0);
}

export const load: LayoutLoad = async (event) => {
  let { session, supabase } = await event.parent()

  const model_id = event.params.id;

  if (session) {
    if (browser) {
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
