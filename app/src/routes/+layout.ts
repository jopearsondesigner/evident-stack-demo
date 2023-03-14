import { browser } from '$app/environment';
import { LayoutLoad } from './$types';

export const load: LayoutLoad = async (event) => {
  if (browser) {
    console.log("loading base +layout.js");
    return {
      dispatch: function(...args) { console.log("Not implemented: ", args) }
    }
  }
};
