import { default as init, EventModelStateManager, event_model_grid } from "app-state";
import { writable, readonly } from 'svelte/store';

const initializeEventModelStore = async (id: string | null | undefined) => {
  let manager = new EventModelStateManager(id);

  let store = writable(await manager.state(), () => {
    return () => console.debug("Unsubscribed last subscriber to Event Model State", id);
  });

  return {
    state: readonly(store),
    dispatch: async (command: any) => {
      console.log("dispatch command:", command, "to manager:", manager)
      let result = await manager.dispatch(command)
      store.set(result)
      console.log("dispatch result:", result)
      return result
    }
  };
}

export const eventModelGrid = (state: any) => {
  if (state.EventModel) {
    return event_model_grid(state.EventModel)
  } else {
    return {}
  }
}

const initWasm = async () => {
  await init();
}

export { initWasm, initializeEventModelStore };
