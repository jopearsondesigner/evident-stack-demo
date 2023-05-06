import type { SupabaseClient } from '@supabase/supabase-js'
import { Dexie } from "dexie";
import type { IDatabaseChange } from 'dexie-observable/api';
import 'dexie-syncable';

// Supabase Syncable

const PATCHES_CHANNEL_PREFIX = "patches";

const enum DatabaseChangeType {
  Create = 1,
  Update = 2,
  Delete = 3,
}

const dexie_change_to_postgrest_obj = (change: IDatabaseChange) => {

};

// `url` will be the model UUID
Dexie.Syncable.registerSyncProtocol("evidentstack", {
  sync: function (_context, url, options, baseRevision, syncedRevision, changes, partial, applyRemoteChanges, onChangesAccepted, onSuccess, onError) {
    let supabase: SupabaseClient = options.supabase;
    const channel = supabase.channel(`${PATCHES_CHANNEL_PREFIX}-${url}`)

    const send_changes = (
      new_changes: typeof changes,
      _base_revision: typeof baseRevision,
      _is_partial: typeof partial,
      on_changes_accepted: typeof onChangesAccepted
    ) => {
      let changes = new_changes.reduce(
        (acc, change) => {
          switch (change.type) {
            case 1: if (change.table == options.models_table) {
              acc.model_upserts.push(change.obj);
            } else if (change.table == options.patches_table) {
              acc.patch_insertions.push(change.obj);
            };
            case 2: acc;
            case 3: acc;
          }
          return acc;
        },
        {
          patch_insertions: [] as any[],
          patch_deletions: [] as any[],
          model_upserts: [] as any[],
          model_deletions: [] as any[]
        }
      );
      supabase.from(options.patches_table).insert(changes.patch_insertions)
        .then(({ error }) => {
          if (error) {
            // on a primary key collision, continue since server already has that patch
            onError(error, Infinity); // retry?
          }
        });
      supabase.from(options.patches_table).update(changes.updates)
        .then(({ error }) => {
          if (error) {
            onError(error, Infinity); // retry?
          }
        });
      supabase.from(options.patches_table).delete().in('id', changes.deletions)
        .then(({ error }) => {
          if (error) {
            onError(error, Infinity); // retry?
          }
        });

      on_changes_accepted();
    };

    const cleanup = () => {
      supabase.removeChannel(channel);
    };

    // 1. query patches since syncedRevision offset, convert them to changes and invoke applyRemoteChanges
    supabase.from(options.patches_table)
      .select()
      .gt('offset', syncedRevision)
      .then(({ error, data }) => {
        // applyRemoteChanges()
        if (error) {
          onError(error, Infinity); // retry?
        }
      });

    // 2. call onSuccess w/ react continuation for future local changes
    onSuccess({ react: send_changes, disconnect: cleanup });

    // 3. react to postgres changes on channel by calling applyRemoteChanges
    channel
      .on('postgres_changes', { event: 'INSERT', schema: options.schema, table: options.patches_table },
        payload => {
          let inserted = payload['new'];
          // applyRemoteChanges()
        })
      // TODO: monitor updates to model metadata table
      .on('postgres_changes', { event: 'DELETE', schema: options.schema, table: options.patches_table },
        payload => {
          let deleted = payload['old'];
          // applyRemoteChanges()
        });

    // 4. send current local changes to server via supabase postgrest client, then call top-level onChangesAccepted()
    send_changes(changes, baseRevision, partial, onChangesAccepted);
  }
});
