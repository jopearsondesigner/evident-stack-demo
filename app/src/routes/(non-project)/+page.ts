import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

// Can't use SSR with dexie-observable
export const ssr = false;

export const load: PageLoad = async (event) => {
  let { session } = await event.parent()
  if (session) {
    const { models_for_user } = await import("$lib/state/dexie");
    const projects = await models_for_user(session.user.id);
    return { projects }
  }
  throw error(404, "not found")
}
