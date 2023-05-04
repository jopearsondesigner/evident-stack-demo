import { Dexie, liveQuery, type Observable } from "dexie";
import { derived, readable, type Readable } from "svelte/store";

type Model = {
  id: string,
  user: string,
  name: string,
  description: string
}

type Patch = {
  id?: number,
  user: string,
  model: string,
  data: Uint8Array
}

type SentPatch = {
  user: string,
  model: string,
  patch_id: number
}

class EventModelDatabase extends Dexie {
  local_patches!: Dexie.Table<Patch, number>;
  remote_patches!: Dexie.Table<Patch, number>;
  sent_patches!: Dexie.Table<SentPatch, [string, string]>;
  models!: Dexie.Table<Model, string>;

  constructor() {
    super("evidentstack");
    this.version(1).stores({
      models: "&id, user, name",
      local_patches: "id++, [model+user]",
      remote_patches: "id++, [model+user]",
      sent_patches: "&[model+user]",
    });
  }
}

const db = new EventModelDatabase();

// TODO: ensure we handle upgrades blocked by other open tabs/windows
// db.on("blocked", function() {
//     alert ("Database upgrading was blocked by another window. " +
//            "Please close down any other tabs or windows that has this page open");
// });

const concatBuffers = (buf1: Uint8Array, buf2: Uint8Array) => {
  let ret = new Uint8Array(buf1.length + buf2.length);
  ret.set(buf1);
  ret.set(buf2, buf1.length);
  return ret;
};

const patchesObservable = (model: string, user: string): Observable<Patch[]> => {
  return liveQuery(() => db.local_patches.where({ model, user }).toArray())
}

export const compositePatchStore = (model: string, user: string) => {
  return readable(
    { id: 0, model, user, data: new Uint8Array() },
    setter => {
      let subscription = patchesObservable(model, user).subscribe(patches => {
        let data = patches.reduce(
          (acc: Uint8Array, patch: Patch) => concatBuffers(acc, patch.data),
          new Uint8Array()
        );
        setter({
          id: patches[patches.length - 1].id || 0,
          user,
          model,
          data
        })
      });

      () => subscription.unsubscribe()
    })
}

export const patches = async (model: string, user: string, starting_at: number | undefined): Promise<Array<Patch>> => {
  let patches = db.local_patches.where({ model, user })
  if (starting_at) {
    return await patches.and(patch => patch.id! > starting_at).toArray()
  } else {
    return await patches.toArray()
  }
}

export const save = async (model: Model, patch: Patch) => {
  let model_dto = { id: model.id, user: model.user, name: model.name, description: model.description }
  let patch_dto = { user: patch.user, model: patch.model, data: patch.data }
  await db.transaction('rw', [db.models, db.local_patches], async () => {
    db.models.put(model_dto, model_dto.id);
    db.local_patches.add(patch_dto)
  });
}

export const save_remote_patch = async (patch: Patch) => {
  let patch_dto = { user: patch.user, model: patch.model, data: patch.data }
  await db.remote_patches.add(patch_dto);
}

export const import_remote_patches = async (model: string, user: string) => {
  await db.transaction('rw', [db.local_patches, db.remote_patches], async () => {
    db.remote_patches.where({ model, user }).each((patch) => {
      if (patch.id) {
        let remote_patch_id = patch.id
        delete patch.id
        db.remote_patches.delete(remote_patch_id)
        db.local_patches.add(patch)
      }
    });
  });
}

export const save_latest_sent_patch = async (patch: SentPatch) => {
  await db.sent_patches.put(patch, [patch.model, patch.user])
}
