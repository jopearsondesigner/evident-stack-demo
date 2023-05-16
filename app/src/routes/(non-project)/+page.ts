import { browser } from "$app/environment";
import { models_for_user } from "$lib/state/dexie";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async (event) => {
  let { session, supabase } = await event.parent()
  if (browser) {
    if (session) {
      const projects = await models_for_user(session.user.id);
      return { projects }
    }
    throw error(404, "not found")
  } else {
    const _projects = await supabase.from("models").select();
    // TODO: map `_projects` from remote into Model[]
    return { projects:  []}
  }
}
