import { Dexie } from "dexie";

type Model = {
  id: string,
  user: string,
  name: string,
  description: string
}

type Patch = {
  user: string,
  model: string,
  data: Uint8Array
}

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
  return await db.patches.where({model: id, user: user}).toArray()
}

export const save = async (model: Model, patch: Patch) => {
  let model_dto = {id: model.id, user: model.user, name: model.name, description: model.description}
  let patch_dto = {user: patch.user, model: patch.model, data: patch.data}
  await db.transaction('rw', [db.models, db.patches], async () => {
    db.models.put(model_dto, model_dto.id);
    db.patches.add(patch_dto)
  });
}
