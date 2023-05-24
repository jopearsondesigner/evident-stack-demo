import { browser } from "$app/environment";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async (event) => {
  let { session } = await event.parent()
  if (browser) {
    const { models_for_user } = await import("$lib/state/dexie");
    if (session) {
      const projects = await models_for_user(session.user.id);
      return { projects }
    }
    throw error(404, "not found")
  } else {
    // TODO: map `_projects` from remote into Model[]
    // const projects = await supabase.from("models").select();
    return { projects:  []}
  }
}
