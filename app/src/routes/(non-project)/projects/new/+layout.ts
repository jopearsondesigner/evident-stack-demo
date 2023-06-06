import { initialize_decider } from '$lib/state/event_model';
import { error } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

// Can't use SSR with dexie-observable
export const ssr = false;

export const load: LayoutLoad = async (event) => {
  let { session } = await event.parent()
  if (session) {
    let { decider } = await initialize_decider(undefined, session.user.id);
    return { create_model: decider.create_model };
  }
  throw error(404, { message: "not found" });
};
