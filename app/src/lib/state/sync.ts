import { debug } from '$lib/util';
import type { Session } from '@supabase/supabase-js';
import { readable, type Readable } from 'svelte/store';
import SyncWorker from '$lib/state/sync.worker?worker';

export const loadSyncWorker = async (session: Session): Promise<Readable<string>> => {
  const worker = new SyncWorker();
  const url = window.location.origin;

  let store = readable("before connection", setter => {
    worker.onmessage = (e) => {
      let status: string = e.data?.status;
      if (status) {
        setter(status);
      }
    };
  });

  debug("Posting init message to sync worker", url, session, worker);

  worker.postMessage({url, session});

  return store
}
