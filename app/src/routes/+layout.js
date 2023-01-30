import { browser } from '$app/environment';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
  if (browser) {
    console.log("loading base +layout.js");
    return {
      dispatch: function(...args) { console.log("Not implemented: ", args) }
    }
  }
}
