import type { ISyncProtocol } from 'dexie-syncable/api';
import type { ICreateChange, IUpdateChange } from 'dexie-observable/api';
import { fromByteArray } from 'base64-js';
import { supabase } from "$lib/supabase/init";

// Supabase Syncable

const CHANNEL_PREFIX = "model";

const dexie_change_to_model_obj = (change: ICreateChange | IUpdateChange) => {
  console.log("model change", change)
  let obj = change.obj
  return { id: obj.id, name: obj.name, description: obj.description }
};

const dexie_change_to_patch_obj = (change: ICreateChange) => {
  console.log("patch insertion", change)
  let obj = change.obj
  return { id: obj.id, model: obj.model, user: obj.user, data: fromByteArray(obj.data) }
};

const CREATE = 1;
const UPDATE = 2;
const DELETE = 3;

// `url` is the model UUID
export const SupabaseSync: ISyncProtocol = {
  sync: function (_context, url, options, baseRevision, syncedRevision, changes, partial,
    applyRemoteChanges, onChangesAccepted, onSuccess, onError) {
    console.debug("Syncing:", url, options, baseRevision, syncedRevision, changes, partial)

    const channel = supabase.channel(`${CHANNEL_PREFIX}-${url}`)

    const send_changes = (
      new_changes: typeof changes,
      _base_revision: typeof baseRevision,
      _is_partial: typeof partial,
      on_changes_accepted: typeof onChangesAccepted
    ) => {
      let changes = new_changes.reduce(
        (acc, change) => {
          switch (change.type) {
            case CREATE: if (change.table == options.models_table) {
              acc.model_insertions.push(dexie_change_to_model_obj(change));
            } else if (change.table == options.patches_table) {
              // Append patch
              acc.patch_insertions.push(dexie_change_to_patch_obj(change));
            }; break;
            case UPDATE: if (change.table == options.models_table) {
              // Model metadata
              acc.model_updates.push(dexie_change_to_model_obj(change));
            }; break;
            case DELETE: if (change.table == options.models_table) {
              // TODO: invoke a deletion function here instead of direct DB modification?
              acc.model_deletions.push(change.key);
            }; break
          }
          return acc;
        },
        {
          patch_insertions: [] as any[],
          model_insertions: [] as any[],
          model_updates: [] as any[],
          model_deletions: [] as any[]
        });

      Promise.all([
        supabase.from(options.patches_table).insert(changes.patch_insertions)
          .then(({ error }) => {
            if (error) {
              // TODO: if model w/ id doesn't exist error
              // (or we don't have permissions to write a patch for a given model id), we should delete the local model
              // on a primary key collision, continue since server already has that patch
              // TODO: also broadcast on a channel to reduce latency?
              onError(error, Infinity); // retry?
            }
          }),
        // TODO: if model w/ id doesn't exist error (or we don't have permissions to write a patch for a given model id), we should delete the local model
        supabase.from(options.models_table).update(changes.model_updates)
          .then(({ error }) => {
            if (error) {
              // TODO: invoke the create model server-side command, for role insertion
              onError(error, Infinity); // retry?
            }
          }),
        supabase.from(options.patches_table).delete().in('id', changes.model_deletions)
          .then(({ error }) => {
            if (error) {
              onError(error, Infinity); // retry?
            }
          })
      ]).then(_result => on_changes_accepted())
    };

    const cleanup = () => {
      supabase.removeChannel(channel);
    };

    // 1. query patches since syncedRevision offset, convert them to changes and invoke applyRemoteChanges
    supabase.from(options.patches_table)
      .select()
      .eq('model', url)
      .neq('user', options.user)
      .gt('offset', syncedRevision)
      .then(({ error, data }) => {
        console.log("initial sync remote data", data)
        // applyRemoteChanges()

        // 2. call onSuccess w/ react continuation for future local changes
        onSuccess({ react: send_changes, disconnect: cleanup });

        if (error) {
          onError(error, Infinity); // retry?
        }
      });

    // // 3. react to postgres changes on channel by calling applyRemoteChanges
    channel
      .on('postgres_changes', { event: 'INSERT', schema: options.schema, table: options.patches_table },
        payload => {
          let inserted = payload['new'];
          // applyRemoteChanges()
        })
      .on('postgres_changes', { event: 'DELETE', schema: options.schema, table: options.patches_table },
        payload => {
          let deleted = payload['old'];
          // applyRemoteChanges()
        })
      .on('postgres_changes', { event: 'INSERT', schema: options.schema, table: options.models_table },
        payload => {
          let inserted = payload['new'];
          // applyRemoteChanges()
        })
      .on('postgres_changes', { event: 'UPDATE', schema: options.schema, table: options.models_table },
        payload => {
          let updated = payload['new'];
          // applyRemoteChanges()
        })
      .on('postgres_changes', { event: 'DELETE', schema: options.schema, table: options.models_table },
        payload => {
          let deleted = payload['old'];
          // applyRemoteChanges()
        })


    // 4. send current local changes to server via supabase postgrest client, then call top-level onChangesAccepted()
    send_changes(changes, baseRevision, partial, onChangesAccepted);
  }
};
