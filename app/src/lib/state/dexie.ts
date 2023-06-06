import { init_supabase } from "$lib/supabase/client";
import type { Session } from "@supabase/supabase-js";
import { Dexie, liveQuery, type Observable } from "dexie";
import { readable } from "svelte/store";
import { SupabaseSync } from "./sync_protocol";

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
      models: "&id, user, name, [id+user]",
      model_patches: "$$id, model",
    });
  }
}

let db: EventModelDatabase;

export const initializeDexie = () => {
  Dexie.Syncable.registerSyncProtocol("evidentstack", SupabaseSync)

  db = new EventModelDatabase();

  // TODO: more gracefully handle upgrades blocked by other open tabs/windows
  db.on("blocked", () => {
    alert("Database upgrading was blocked by another window. " +
      "Please close down any other tabs or windows that has this page open");
  });

  return db;
};

const concatBuffers = (buf1: Uint8Array, buf2: Uint8Array) => {
  let ret = new Uint8Array(buf1.length + buf2.length);
  ret.set(buf1);
  ret.set(buf2, buf1.length);
  return ret;
};

const combine_patches = (patches: Patch[]) => {
  return patches.reduce(
    (acc: Uint8Array, patch: Patch) => concatBuffers(acc, patch.data),
    new Uint8Array()
  )
}

const patchesLiveQuery = (model: string): Observable<Patch[]> => {
  return liveQuery(() => db.model_patches.where({ model }).toArray())
}

export const documentBinaryStore = (model: string) => {
  return readable(
    new Uint8Array(),
    setter => {
      let subscription = patchesLiveQuery(model).subscribe(patches => {
        let data = combine_patches(patches);
        setter(data)
      });
      () => subscription.unsubscribe()
    })
}

export const models_for_user = async (user: string) => {
  return await db.models.where({ user }).toArray();
}

export const model_by_id = async (user: string, id: string) => {
  let results = await db.models.where({ id, user }).toArray();
  return results[0];
}

export const model_patches = async (model: string | undefined): Promise<Array<Patch>> => {
  return await db.model_patches.where({ model }).toArray()
}

export const model_binary = async (model: string | undefined) => {
  return combine_patches(await (model_patches(model)))
}

export const save = async (model: Model, patch: Patch) => {
  let model_dto = { id: model.id, user: model.user, name: model.name, description: model.description }
  let patch_dto = { model: patch.model, data: patch.data }
  await db.transaction('rw', [db.models, db.model_patches], async () => {
    db.models.put(model_dto, model_dto.id);
    db.model_patches.add(patch_dto)
  });
}

export const connect = async (url: string, session: Session, statusCallback: (status: number) => void) => {
  init_supabase(session);

  await db.syncable.connect("evidentstack", url,
    {
      user: session.user.id,
      local_patches_table: "model_patches",
      local_models_table: "models",
      remote_schema: "public",            // TODO: configurable?
      remote_events_table: "model_events", // TODO: configurable?
      remote_patches_table: "model_patches" // TODO: configurable?
    }
  );

  db.syncable.on('statusChanged', function (newStatus, url_) {
    console.log("Dexie DB status changing to:", newStatus, Dexie.Syncable.StatusTexts[newStatus]);
    if (url_ == url) {
      statusCallback(newStatus);
    }
  });
}
