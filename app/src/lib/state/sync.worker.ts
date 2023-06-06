import type { Session } from '@supabase/supabase-js';

onmessage = async (e) => {
  const { debug } = await import('$lib/util');
  const { initializeDexie, connect } = await import('$lib/state/dexie');
  await import('dexie-observable');
  await import('dexie-syncable');
  initializeDexie();
  debug("Worker received message", e);
  let { url, session }: { url: string, session: Session } = e.data;
  if (url && session) {
    debug("Worker connecting to sync service", url, session);
    try {
      await connect(url, session, (status) => {
        debug("New connection status on worker:", status);
        postMessage({ status });
      });
    } catch (e) {
      console.error("Error while connecting sync worker", e);
    }
    debug("Worker connected to sync service", url);
  }
};
