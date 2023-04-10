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
        return await manager.create(name)
      },
      define_and_place_interface: async (name: string, index: number, audience: string | undefined) => {
        return await manager.define_and_place_interface(id!, name, index, audience);
      },
      define_and_place_command: async (name: string, index: number) => {
        return await manager.define_and_place_command(id!, name, index);
      },
      define_and_place_event: async (name: string, index: number, stream: string | undefined) => {
        return await manager.define_and_place_event(id!, name, index, stream);
      },
      define_and_place_read_model: async (name: string, index: number) => {
        return await manager.define_and_place_read_model(id!, name, index);
      },
      delete_model: async () => {
        return await manager.delete(id!)
      },
      duplicate_interface_placement: async (placement_id: string, index: number, audience: string | undefined) => {
        return await manager.duplicate_interface_placement(id!, placement_id, index, audience)
      },
      duplicate_timeline_placement: async (placement_id: string, index: number) => {
        return await manager.duplicate_timeline_placement(id!, placement_id, index)
      },
      duplicate_event_placement: async (placement_id: string, index: number, stream: string | undefined) => {
        return await manager.duplicate_event_placement(id!, placement_id, index, stream)
      },
      import_json: async (json_bytes: Uint8Array, offset: number) => {
        return await manager.import(id!, json_bytes, offset)
      },
      move_interface_placement: async (placement_id: string, index: number, audience: string | undefined) => {
        return await manager.move_interface_placement(id!, placement_id, index, audience)
      },
      move_timeline_placement: async (placement_id: string, index: number) => {
        return await manager.move_timeline_placement(id!, placement_id, index)
      },
      move_event_placement: async (placement_id: string, index: number, stream: string | undefined) => {
        return await manager.move_event_placement(id!, placement_id, index, stream)
      },
      remove_placement: async (placement: string) => {
        return await manager.remove_placement(id!, placement)
      },
      rename_placement: async (placement: string, name: string) => {
        return await manager.rename_placement(id!, placement, name)
      },
    }
  };
}

export { initialize_decider };
