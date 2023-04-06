import { initialize_decider } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (_event) => {
  if (browser) {
    let {decider} = await initialize_decider(undefined);
    return {create_model: decider.create_model};
  }
};
