import { default as init, EventModelStateManager } from "app-state";
import { writable, readonly } from 'svelte/store';

const initializeEventModelStore = async (id: string | undefined) => {
  let manager = new EventModelStateManager(id);

  let store = writable(await manager.state(), () => {
    return () => console.debug("Unsubscribed last subscriber to Event Model State", id);
  });

  return {
    state: readonly(store),
    create_model: async (name: string) => {
      let result = await manager.create(name)
      store.set(result)
      return result
    },
    delete_model: async (model_id: string) => {
      let result = await manager.delete(model_id)
      store.set(result)
      return result
    },
    import_json: async (model_id: string, json_bytes: Uint8Array, offset: number) => {
      let result = await manager.import(model_id, json_bytes, offset)
      store.set(result)
      return result
    }
  };
}

const initWasm = async () => {
  await init();
}

export { initWasm, initializeEventModelStore };
