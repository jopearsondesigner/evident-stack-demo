import { debug } from '$lib/util';
import type { Session } from '@supabase/supabase-js';
import { readable, type Readable } from 'svelte/store';
import SyncWorker from '$lib/state/sync.worker?worker';

export const loadSyncWorker = async (session: Session): Promise<Readable<number>> => {
  const worker = new SyncWorker();
  const url = window.location.origin;

  let store = readable(0, setter => {
    worker.onmessage = (e) => {
      let sync_status: number = e.data?.status;
      if (sync_status) {
        setter(sync_status);
      }
    };
  });

  debug("Posting init message to sync worker", url, session, worker);

  worker.postMessage({url, session});

  return store
}
