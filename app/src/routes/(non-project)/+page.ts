import { error, redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { models_for_user as remote_models_for_user } from "$lib/supabase/database";
import { browser } from "$app/environment";

export const load: PageLoad = async (event) => {
  let { session, supabase } = await event.parent()
  if (session) {
    let projects: { id: string, name: string, description: string, user: string }[];
    try {
      const user_id = session.user.id;
      const models = await remote_models_for_user(supabase);
      projects = models.map((m) => {
        return { user: user_id, ...m, description: m.description || "" }
      });
    } catch (e) {
      if (browser) {
        const { models_for_user } = await import("$lib/state/dexie");
        projects = await models_for_user(session.user.id);
      } else {
        projects = [];
      }
    }
    return { projects }
  }
  throw redirect(303, '/auth');
}
