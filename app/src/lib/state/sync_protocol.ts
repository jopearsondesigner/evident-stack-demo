import type { ISyncProtocol } from 'dexie-syncable/api';
import type { IDatabaseChange, ICreateChange, IUpdateChange } from 'dexie-observable/api';
import { toByteArray, fromByteArray } from 'base64-js';
import type { Database } from "$lib/supabase/database.types";
import type { SupabaseClient } from "@supabase/supabase-js";
import { debug } from '$lib/util';

const CREATE = 1;
const UPDATE = 2;
const DELETE = 3;

const INITIAL_BACKOFF = 2;
let local_to_remote_backoff = INITIAL_BACKOFF;
let remote_to_local_backoff = INITIAL_BACKOFF;


const dexie_change_to_model_obj = (change: ICreateChange | IUpdateChange) => {
  let obj = change.obj
  return { id: obj.id, name: obj.name, description: obj.description }
};

const dexie_change_to_patch_obj = (change: ICreateChange) => {
  let obj = change.obj
  return { id: obj.id, model: obj.model, user: obj.user, data: fromByteArray(obj.data) }
};

export let supabase: SupabaseClient<Database>;

export const initSupabase = (client: SupabaseClient<Database>): void => {
  supabase = client;
}

export const SupabaseSync: ISyncProtocol = {
  sync: async function (context, url, options, baseRevision, syncedRevision, changes, partial,
    applyRemoteChanges, onChangesAccepted, onSuccess, onError) {
    debug("Syncing:", url, options, baseRevision, syncedRevision, changes, partial)

    const send_changes = async (
      new_changes: typeof changes,
      _base_revision: typeof baseRevision,
      _is_partial: typeof partial,
      on_changes_accepted: typeof onChangesAccepted
    ) => {
      debug("Preparing to send changes to Supabase:", new_changes);

      let changes = new_changes.reduce(
        (acc, change) => {
          switch (change.type) {
            case CREATE: if (change.table == options.local_models_table) {
              acc.model_insertions.push(dexie_change_to_model_obj(change));
            } else if (change.table == options.local_patches_table) {
              acc.patch_insertions.push(dexie_change_to_patch_obj(change));
            }; break;
            case UPDATE: if (change.table == options.local_models_table) {
              // acc.model_updates.push(dexie_change_to_model_obj(change));
            }; break;
            case DELETE: if (change.table == options.local_models_table) {
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

      // TODO: ensure we don't send empty changes?
      debug("applying local changes to Supabase", changes);
      let { error, ...response } = await supabase.rpc("apply_client_changes", { changes });
      debug("apply_client_changes response:", { error, ...response })
      if (error) {
        // TODO: if model w/ id doesn't exist error
        // (or we don't have permissions to write a patch for a given model id), we should delete the local model
        console.warn("Error applying local changes to remote server:", error);
        local_to_remote_backoff *= 2;
        onError(error, local_to_remote_backoff);
      } else {
        // TODO: also broadcast on a channel to reduce latency?
        local_to_remote_backoff = INITIAL_BACKOFF;
        on_changes_accepted()
      }
    };

    const model_event_to_dexie_change = (event: any): IDatabaseChange[] => {
      switch (event.type) {
        case 'created': return [{
          type: CREATE,
          table: options.local_models_table,
          key: event.subject,
          obj: { user: event.user, ...event.data },
        }];
        case 'updated': return [{
          type: UPDATE,
          table: options.local_models_table,
          key: event.subject,
          mods: { user: event.user, ...event.data },
          obj: { user: event.user, ...event.data },
          oldObj: null,
        }];
        case 'deleted': return [{
          type: DELETE,
          table: options.local_models_table,
          key: event.subject,
          oldObj: null,
        }];
        case 'patched': return [{
          type: CREATE,
          table: options.local_patches_table,
          key: event.subject,
          obj: { id: event.data.patch_id, model: event.subject, data: toByteArray(event.patch_data) }
        }];
        // nothing to do here, right? A newly granted collaborator should receive an initial,
        // full sync of all events
        case 'collaborator_role_granted': return [];
        case 'collaborator_role_revoked': return [{
          type: DELETE,
          table: options.local_models_table,
          key: event.subject,
          oldObj: null,
        }];
        case 'snapshotted': return [{
          type: CREATE,
          table: options.local_patches_table,
          key: event.subject,
          obj: { id: event.data.patch_id, model: event.subject, data: toByteArray(event.patch_data) }
        }, ...event.data.obsolete_patch_ids.map((id: string) => {
          return {
            type: DELETE,
            table: options.local_patches_table,
            key: id,
            oldObj: null,
          }
        }
        )];
      }
      return [];
    };

    // 1. react to postgres changes on channel by calling applyRemoteChanges
    // (do this first for at-least-once semantics applying remote events to local)
    debug("Subscribing to postgres_changes")
    const channel = supabase.channel('evidentsystems-model-events')
    channel
      .on('postgres_changes',
        {
          event: 'INSERT',
          schema: options.remote_schema,
        },
        async payload => {
          if (payload.table == options.remote_patches_table) {
            debug("received model patch via postgres_changes on channel", payload)
            // We should receive patches first, since we write patches
            // before events in the SQL decider functions, so we store
            // it in the context for use by the associated event,
            // which should be the next message received
            let patch = payload['new'];
            debug("storing unapplied patch", patch);
            context.unapplied_patches = {...context.unapplied_patches, [patch.id]: patch}
            await context.save();
          } else if (payload.table == options.remote_events_table) {
            debug("received model event via postgres_changes on channel", payload)
            let event = payload['new'];
            if (event.type == 'patched' || event.type == 'snapshotted') {
              // Get the patch data from the previously stored unapplied_patch on the context
              let patch = context.unapplied_patches[event.data.patch_id];
              delete context.unapplied_patches[event.data.patch_id];
              debug("looked up patch data from context", patch);
              event.patch_data = patch.data;
              await applyRemoteChanges(model_event_to_dexie_change(event), event.id)
              await context.save()
            } else {
              await applyRemoteChanges(model_event_to_dexie_change(event), event.id)
            }
          }
        })
      .subscribe()
    const cleanup = () => {
      debug("cleaning up Supabase channel on disconnect", channel)
      supabase.removeChannel(channel);
    };

    debug("Fetching events since event id:", syncedRevision)
    // 2. query events since syncedRevision offset, convert them to changes and invoke applyRemoteChanges
    let { error, data, ...response } = await supabase.rpc(
      "model_events_since",
      { starting_event_id: syncedRevision }
    )
    debug("Received events: since event id:", syncedRevision, "response:", { error, data, ...response })
    if (error) {
      console.warn("Error fetching remote changes from remote server:", error);
      remote_to_local_backoff *= 2;
      onError(error, remote_to_local_backoff);
    } else {
      remote_to_local_backoff = INITIAL_BACKOFF;
      const changes = data?.flatMap(model_event_to_dexie_change) || [];
      const revision = data?.length && data.length > 0 ? data[data.length - 1].id : null;

      debug("initial sync remote data", data, "as local changes", changes, "as revision", revision);

      await applyRemoteChanges(changes, revision);

      // 3. call onSuccess w/ react continuation for future local changes
      onSuccess({ react: send_changes, disconnect: cleanup });
    }

    // 4. send current local changes to server via supabase postgrest client,
    // then call top-level onChangesAccepted()
    send_changes(changes, baseRevision, partial, onChangesAccepted);
  }
};
