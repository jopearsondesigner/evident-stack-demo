import { browser } from "$app/environment";
import type { SupabaseClient } from "@supabase/supabase-js";
import type { Database } from "$lib/supabase/database.types";
import { Dexie, liveQuery, type Observable } from "dexie";
import { readable, type Readable } from "svelte/store";

if (browser) {
  await import('dexie-observable');
  await import('dexie-syncable');
  const { SupabaseSync } = await import("./sync_protocol");
  Dexie.Syncable.registerSyncProtocol("evidentstack", SupabaseSync)
}

type Model = {
  id: string,
  user: string,
  name: string,
  description: string
}

type Patch = {
  id?: string,
  model: string,
  data: Uint8Array
}

class EventModelDatabase extends Dexie {
  model_patches!: Dexie.Table<Patch, string>;
  models!: Dexie.Table<Model, string>;

  constructor() {
    super("evidentstack");
    this.version(1).stores({
      models: "&id, user, name",
      model_patches: "$$id, model",
    });
  }
}

export const db = new EventModelDatabase();

export const connect = async (supabase: SupabaseClient<Database>): Promise<Readable<string>> => {
  if (browser) {
    const { initSupabase } = await import("./sync_protocol");
    initSupabase(supabase);

    const url = window.location.origin;

    await db.syncable.connect("evidentstack", url,
      {
        local_patches_table: "model_patches",
        local_models_table: "models",
        remote_schema: "public",            // TODO: configurable?
        remote_events_table: "model_events", // TODO: configurable?
        remote_patches_table: "model_patches" // TODO: configurable?
      }
    );

    // Return a store that tracks connection status
    return readable("before connection", setter => {
      db.syncable.on('statusChanged', function (newStatus, url_) {
        console.log("Dexie DB status changing to:", newStatus, Dexie.Syncable.StatusTexts[newStatus]);
        if (url_ == url) {
          setter(Dexie.Syncable.StatusTexts[newStatus]);
        }
      });
    })
  };

  return readable("not connected")
}

// TODO: more gracefully handle upgrades blocked by other open tabs/windows
db.on("blocked", () => {
  alert("Database upgrading was blocked by another window. " +
    "Please close down any other tabs or windows that has this page open");
});

const concatBuffers = (buf1: Uint8Array, buf2: Uint8Array) => {
  let ret = new Uint8Array(buf1.length + buf2.length);
  ret.set(buf1);
  ret.set(buf2, buf1.length);
  return ret;
};

const patchesObservable = (model: string): Observable<Patch[]> => {
  return liveQuery(() => db.model_patches.where({ model }).toArray())
}

export const documentBinaryStore = (model: string) => {
  return readable(
    new Uint8Array(),
    setter => {
      let subscription = patchesObservable(model).subscribe(patches => {
        let data = patches.reduce(
          (acc: Uint8Array, patch: Patch) => concatBuffers(acc, patch.data),
          new Uint8Array()
        );
        setter(data)
      });
      () => subscription.unsubscribe()
    })
}

export const patches = async (model: string | undefined): Promise<Array<Patch>> => {
  return await db.model_patches.where({ model }).toArray()
}

export const save = async (model: Model, patch: Patch) => {
  let model_dto = { id: model.id, user: model.user, name: model.name, description: model.description }
  let patch_dto = { model: patch.model, data: patch.data }
  await db.transaction('rw', [db.models, db.model_patches], async () => {
    db.models.put(model_dto, model_dto.id);
    db.model_patches.add(patch_dto)
  });
}
