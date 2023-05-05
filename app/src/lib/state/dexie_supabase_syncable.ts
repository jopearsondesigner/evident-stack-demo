import { Dexie } from "dexie";
import 'dexie-syncable';

// Supabase Syncable

Dexie.Syncable.registerSyncProtocol("supabase", {
  sync: function (context, url, options, baseRevision, syncedRevision, changes, partial, applyRemoteChanges, onChangesAccepted, onSuccess, onError) {
    
  }
});
