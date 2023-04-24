import type { Model, Patch } from "app-state";
import { Dexie } from "dexie";

class EventModelDatabase extends Dexie {
  patches!: Dexie.Table<Patch, number>;
  models!:  Dexie.Table<Model, string>;

  constructor () {
        super("evidentstack");
        this.version(1).stores({
          models: "&id, user, name",
          patches: "id++, [model+user]"
        });
    }
}

var db = new EventModelDatabase();

export const patches = async (id: string, user: string): Promise<Array<Patch>> => {
  console.log("patches", id, user)
  return await db.patches.where({model: id, user: user}).toArray()
}

export const save = async (model: Model, patch: Patch) => {
  console.log("saving", model, patch)
  await db.transaction('rw', [db.models, db.patches], async () => {
    db.models.put(model, model.id);
    db.patches.add(patch)
  });
  console.log("...saved")
}
