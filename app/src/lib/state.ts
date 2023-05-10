import { default as init, EventModelGrid, EventModelStateManager, setPanicHook } from "app-state";
import { readable, type Readable } from 'svelte/store';
import type { Decider, LaneKind } from '$components/design/Grid';
import { dev } from "$app/environment";

export type InitializationPayload = {
  grid: Readable<EventModelGrid>,
  decider: Decider
}

const initialize_decider = async (id: string | undefined, user: string) => {
  await init();
  if (dev) {
    setPanicHook();
  }

  let manager = await new EventModelStateManager(id, user);

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
      rename_lane: async (kind: LaneKind, lane_id: string, name: string) => {
        return await manager.rename_lane(id!, kind, lane_id, name)
      },
      reorder_lane: async (kind: LaneKind, lane_id: string, index: number) => {
        return await manager.reorder_lane(id!, kind, lane_id, index)
      },
      remove_lane: async (kind: LaneKind, lane_id: string) => {
        return await manager.remove_lane(id!, kind, lane_id)
      },
      add_to_description: async (index: number, addition: string) => {
        return await manager.add_to_description(id!, index, addition)
      },
      delete_from_description: async (index: number, count: number) => {
        return await manager.delete_from_description(id!, index, count)
      }
      // TODO: Add linking flows
    }
  };
}

export { initialize_decider };
