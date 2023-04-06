import { default as init, EventModelStateManager } from "app-state";
import { readable } from 'svelte/store';

const initialize_decider = async (id: string | undefined) => {
  await init();

  let manager = new EventModelStateManager(id);

  let store = readable(await manager.state(), (setter) => {
    manager.store_setter = setter;
    return () => console.debug("Unsubscribed last subscriber to Event Model State", id);
  });

  return {
    grid: store,
    decider: {
      create_model: async (name: string) => {
        let result = await manager.create(name)
        return result
      },
      define_and_place_interface: async (name: string, index: number, audience: string | undefined) => {
        console.log('define_and_place_interface', name, index, audience)
      },
      define_and_place_command: async (name: string, index: number) => {
        console.log('define_and_place_command', name, index)
      },
      define_and_place_event: async (name: string, index: number, stream: string | undefined) => {
        console.log('define_and_place_event', name, index, stream)
      },
      define_and_place_read_model: async (name: string, index: number) => {
        console.log('define_and_place_read_model', name, index)
      },
      delete_model: async () => {
        let result = await manager.delete(id)
        return result
      },
      import_json: async (json_bytes: Uint8Array, offset: number) => {
        let result = await manager.import(id, json_bytes, offset)
        return result
      },
      rename_placement: (placement: string, name: string) => {
        console.log('rename_placement', placement, name)
      },
    }
  };
}

export { initialize_decider };
