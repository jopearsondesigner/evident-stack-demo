import type { EventModelCommand } from "app-state";
import { default as init, EventModelStateManager } from "app-state";
import { readable, derived } from 'svelte/store';

const initializeEventModelStore = (id: string) => {
  let manager = new EventModelStateManager(id);
  let store = readable(null, setter => {
    manager.initialize(setter);

    return () => console.debug("Unsubscribed last subscriber to Event Model", id);
  });

  return {
    store,
    dispatch: async (command: EventModelCommand) => {
      let result = await manager.dispatch(command)
    }
  };
}

const initWasm = async function() {
    await init();
}

export { initWasm, initializeEventModelStore };
