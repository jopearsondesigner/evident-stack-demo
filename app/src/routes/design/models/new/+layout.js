import { initializeEventModelCreationContext } from '$lib/state';
import { browser } from '$app/environment';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
  if (browser) {
    return initializeEventModelCreationContext();
  }
}
