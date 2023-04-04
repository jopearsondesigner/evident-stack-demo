import { default as init, EventModelStateManager } from "app-state";
import { writable, readonly } from 'svelte/store';

// TODO: add return types - see below
const initializeEventModelStore = async (id: string | undefined) => {
  await init();

  let manager = new EventModelStateManager(id);

  // TODO: Annotate Cast JsValue with EventModel Grid - propagate this all the way out for type hinting when consumed in TS
  let store = writable(await manager.state(), () => {
    return () => console.debug("Unsubscribed last subscriber to Event Model State", id);
  });

  return {
    // TODO: type Readable<EventModelGrid>
    state: readonly(store),
    // TODO: add return type cast
    create_model: async (name: string) => {
      let result = await manager.create(name)
      store.set(result)
      return result
    },
    // TODO: add return type cast
    delete_model: async (model_id: string) => {
      let result = await manager.delete(model_id)
      store.set(result)
      return result
    },
    // TODO: add return type cast
    import_json: async (model_id: string, json_bytes: Uint8Array, offset: number) => {
      let result = await manager.import(model_id, json_bytes, offset)
      store.set(result)
      return result
    }
  };
}

export { initializeEventModelStore };
