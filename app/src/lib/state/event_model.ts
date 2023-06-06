import { default as init, EventModelStateManager, setPanicHook } from "state-client";
import { derived, readable } from 'svelte/store';
import type { ReorderableLaneType } from '$components/design/Grid';

const initialize_decider = async (id: string | undefined, user: string) => {
  // Initialize Wasm decider
  await init();
  setPanicHook();

  let manager = new EventModelStateManager(id, user);

  const { model_binary, documentBinaryStore } = await import("./dexie");

  let store;

  if (id) {
    // Initialize Dexie from existing patches
    let initial_bin = await model_binary(id);
    manager.refresh(initial_bin);

    // Setup store via documentBinaryStore for ongoing refreshes upon change to storage
    let $doc_binary_store = documentBinaryStore(id);

    store = derived($doc_binary_store, (bin, setter) => {
      try {
        manager.refresh(bin);
        manager.grid()
          .then((grid) => {
            setter(grid);
          }).catch((_) => {
            setter(null);
          });
      } catch {
        setter(null);
      }
      return () => console.debug("Unsubscribed last subscriber to empty Event Model State");
    });
  } else {
    store = readable(null, (_) => {
      return () => console.debug("Unsubscribed last subscriber to empty Event Model State");
    })
  }

  return {
    grid: store,
    decider: {
      placement_by_id: async (placement_id: string) => {
        let grid = await manager.grid();
        return grid.placement_by_id(placement_id);
      },
      create_model: async (name: string) => {
        return await manager.create(name);
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
      rename_lane: async (kind: ReorderableLaneType, lane_id: string, name: string) => {
        return await manager.rename_lane(id!, kind, lane_id, name)
      },
      reorder_lane: async (kind: ReorderableLaneType, lane_id: string, index: number) => {
        return await manager.reorder_lane(id!, kind, lane_id, index)
      },
      remove_lane: async (kind: ReorderableLaneType, lane_id: string) => {
        return await manager.remove_lane(id!, kind, lane_id)
      },
      add_lane: async (kind: string, index: number, name: string) => {
        console.warn("add lane", { kind, index , name });
        return manager.add_lane(id!, kind, index, name);
      },
      insert_columns: async (index: number, direction: string, count: number) => {
        return manager.insert_columns(id!, index, direction, count);
      },
      add_to_description: async (index: number, addition: string) => {
        return await manager.add_to_description(id!, index, addition)
      },
      delete_from_description: async (index: number, count: number) => {
        return await manager.delete_from_description(id!, index, count)
      },
      connect_flow: async (source_placement_id_str: string, source_anchor_str: string | undefined, target_placement_id_str: string, target_anchor_str: string | undefined) => {
        return await manager.connect_flow(id!, source_placement_id_str, source_anchor_str, target_placement_id_str, target_anchor_str);
      },
    }
  };
}

export { initialize_decider };
