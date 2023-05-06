import { browser } from "$app/environment";
import { Dexie, liveQuery, type Observable } from "dexie";
import { readable } from "svelte/store";

if (browser) {
  await import('dexie-observable');
  await import('dexie-syncable');
}

type Model = {
  id: string,
  user: string,
  name: string,
  description: string
}

type Patch = {
  id?: string,
  user: string,
  model: string,
  data: Uint8Array
}

class EventModelDatabase extends Dexie {
  patches!: Dexie.Table<Patch, string>;
  models!: Dexie.Table<Model, string>;

  constructor() {
    super("evidentstack");
    this.version(1).stores({
      models: "&id, user, name",
      patches: "$$id, [model+user]",
    });
  }
}

const db = new EventModelDatabase();

// TODO: more gracefully handle upgrades blocked by other open tabs/windows
db.on("blocked", function() {
  alert ("Database upgrading was blocked by another window. " +
    "Please close down any other tabs or windows that has this page open");
});

const concatBuffers = (buf1: Uint8Array, buf2: Uint8Array) => {
  let ret = new Uint8Array(buf1.length + buf2.length);
  ret.set(buf1);
  ret.set(buf2, buf1.length);
  return ret;
};

const patchesObservable = (model: string, user: string): Observable<Patch[]> => {
  return liveQuery(() => db.patches.where({ model, user }).toArray())
}

export const documentBinaryStore = (model: string, user: string) => {
  return readable(
    new Uint8Array(),
    setter => {
      let subscription = patchesObservable(model, user).subscribe(patches => {
        let data = patches.reduce(
          (acc: Uint8Array, patch: Patch) => concatBuffers(acc, patch.data),
          new Uint8Array()
        );
        setter(data)
      });
      () => subscription.unsubscribe()
    })
}

export const patches = async (model: string, user: string | undefined): Promise<Array<Patch>> => {
  return await db.patches.where({ model, user }).toArray()
}

export const save = async (model: Model, patch: Patch) => {
  let model_dto = { id: model.id, user: model.user, name: model.name, description: model.description }
  let patch_dto = { user: patch.user, model: patch.model, data: patch.data }
  await db.transaction('rw', [db.models, db.patches], async () => {
    db.models.put(model_dto, model_dto.id);
    db.patches.add(patch_dto)
  });
}
