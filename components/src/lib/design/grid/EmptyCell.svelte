<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { DragEventHandler } from 'svelte/elements';
  import type { CellType } from '../Grid';

  export let column: number;
  export let kind: CellType;
  export let lane: string | undefined = undefined;

  const dispatch = createEventDispatcher();

  let drop_target: 'target' | 'bad-target' | 'none' = 'none';

  $: good_target = drop_target == 'target'
  $: bad_target = drop_target == 'bad-target'

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer && (transfer.effectAllowed == 'copy' || transfer.effectAllowed == 'move')) {
      if (transfer.types.includes(kind)) {
        e.preventDefault();
        transfer.dropEffect = transfer.effectAllowed == 'copy' ? 'copy' : 'move';
        drop_target = 'target';
      } else {
        transfer.dropEffect = 'none';
        drop_target = 'bad-target';
      }
    }
  }

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (_e) => {
    drop_target = 'none';
  }

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
    handleDragLeave(e);
    let transfer = e.dataTransfer;
    let id = transfer?.getData(kind);
    if (transfer && id && (transfer.effectAllowed == 'copy' || transfer.effectAllowed == 'move')) {
      transfer.dropEffect = transfer.effectAllowed == 'copy' ? 'copy' : 'move';
      if (kind == 'interface') {
        if (transfer.dropEffect == 'move') {
          dispatch('move_interface_placement', {id: id, index: column, audience: lane})
        } else if (transfer.dropEffect == 'copy') {
          dispatch('duplicate_interface_placement', {id: id, index: column, audience: lane})
        }
      } else if (kind == 'timeline') {
        if (transfer.dropEffect == 'move') {
          dispatch('move_timeline_placement', {id: id, index: column})
        } else if (transfer.dropEffect == 'copy') {
          dispatch('duplicate_timeline_placement', {id: id, index: column})
        }
      } else if (kind == 'event') {
        if (transfer.dropEffect == 'move') {
          dispatch('move_event_placement', {id: id, index: column, stream: lane})
        } else if (transfer.dropEffect == 'copy') {
          dispatch('duplicate_event_placement', {id: id, index: column, stream: lane})
        }
      }
    }
  }
</script>

<div
  on:dragenter={handleDragEnter}
  on:dragover={(e) => {e.preventDefault()}}
  on:dragleave={handleDragLeave}
  on:drop={handleDragDrop}
  class:bg-emerald-200={good_target}
  class:bg-rose-400={bad_target}
  class="empty-cell min-w-placementPadded min-h-placementPadded" />
