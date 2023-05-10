import type { ISyncProtocol } from 'dexie-syncable/api';
import type { IDatabaseChange, ICreateChange, IUpdateChange } from 'dexie-observable/api';
import { toByteArray, fromByteArray } from 'base64-js';
import { dev } from '$app/environment';
import type { Database } from "$lib/supabase/database.types";
import type { SupabaseClient } from "@supabase/supabase-js";

const CHANNEL_PREFIX = "model";

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

const debug = (...args: any[]) => {
  if (dev) {
    console.debug(...args)
  }
}

export let supabase: SupabaseClient<Database>;

export const initSupabase = (client: SupabaseClient<Database>): void => {
  supabase = client;
}

// `url` is the model UUID
export const SupabaseSync: ISyncProtocol = {
  sync: async function (_context, url, options, baseRevision, syncedRevision, changes, partial,
    applyRemoteChanges, onChangesAccepted, onSuccess, onError) {
    debug("Syncing:", url, options, baseRevision, syncedRevision, changes, partial)
    const channel = supabase.channel(`${CHANNEL_PREFIX}-${url}`)

    const send_changes = async (
      new_changes: typeof changes,
      _base_revision: typeof baseRevision,
      _is_partial: typeof partial,
      on_changes_accepted: typeof onChangesAccepted
    ) => {
      let changes = new_changes.reduce(
        (acc, change) => {
          switch (change.type) {
            case CREATE: if (change.table == options.local_models_table) {
              acc.model_insertions.push(dexie_change_to_model_obj(change));
            } else if (change.table == options.local_patches_table) {
              acc.patch_insertions.push(dexie_change_to_patch_obj(change));
            }; break;
            case UPDATE: if (change.table == options.local_models_table) {
              acc.model_updates.push(dexie_change_to_model_obj(change));
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

      debug("applying local changes to Supabase", changes, "by user:", await supabase.auth.getUser());
      let { error, ...response } = await supabase.rpc("apply_client_changes", { changes });
      debug("apply_client_changes response:", { error, ...response })
      if (error) {
        // TODO: if model w/ id doesn't exist error
        // (or we don't have permissions to write a patch for a given model id), we should delete the local model
        // TODO: on a primary key collision, continue since server already has that patch
        local_to_remote_backoff *= 2;
        onError(error, local_to_remote_backoff);
      } else {
        // TODO: also broadcast on a channel to reduce latency?
        local_to_remote_backoff = INITIAL_BACKOFF;
        on_changes_accepted()
      }
    };

    const cleanup = () => {
      debug("cleaning up Supabase channel on disconnect", channel)
      supabase.removeChannel(channel);
    };

    const model_event_to_dexie_change = (event: any): IDatabaseChange[] => {
      debug("mapping model_event to a Dexie change", event)
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

    debug("Fetching events for model:", url,
      "by user:", await supabase.auth.getUser(),
      "since event id:", syncedRevision)
    // 1. query events since syncedRevision offset, convert them to changes and invoke applyRemoteChanges
    let { error, data, ...response } = await supabase.rpc(
      "model_events_since",
      { model_id: url, starting_event_id: syncedRevision }
    )
    debug("Received events for model:", url,
      "since event id:", syncedRevision,
      "response:", { error, data, ...response })
    if (error) {
      remote_to_local_backoff *= 2;
      onError(error, remote_to_local_backoff);
    } else {
      remote_to_local_backoff = INITIAL_BACKOFF;
      const changes = data?.flatMap(model_event_to_dexie_change) || [];

      debug("initial sync remote data", data, "as local changes", changes)

      applyRemoteChanges(changes, data?.length && data.length > 0 ? data[data.length - 1].id : null, false, false)

      // 2. call onSuccess w/ react continuation for future local changes
      onSuccess({ react: send_changes, disconnect: cleanup });
    }

    // 3. react to postgres changes on channel by calling applyRemoteChanges
    // TODO: there is a small window for permenantly missing an event between query above and first conveyed event!!!
    //   should we just rely on broadcast channel for incremental patches
    channel
      .on('postgres_changes',
        {
          event: 'INSERT',
          schema: options.remote_schema,
          table: options.remote_events_table,
          filter: `subject=eq.${url}`
        },
        payload => {
          let event = payload['new'];
          debug("received model event from remote on channel", payload)
          applyRemoteChanges(model_event_to_dexie_change(event), event.id, false, false)
        })

    // 4. send current local changes to server via supabase postgrest client,
    // then call top-level onChangesAccepted()
    send_changes(changes, baseRevision, partial, onChangesAccepted);
  }
};
