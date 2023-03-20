import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (_event) => {
  if (browser) {
    console.log("loading base +layout.js");
    return {
      dispatch: function(...args: [any]) { console.log("Not implemented: ", args) }
    }
  }
};
