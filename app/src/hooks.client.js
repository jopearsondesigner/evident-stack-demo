import 'dexie-observable';
import 'dexie-syncable';
import { initializeDexie } from '$lib/state/dexie';

initializeDexie();

/** @type {import('@sveltejs/kit').HandleClientError} */
export function handleError({ error, event }) {
  console.error('Client error:', error);

  return {
    message: 'Application error'
  };
}
