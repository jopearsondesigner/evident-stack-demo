<script lang="ts">
  import MaybeTooltip from '../../utils/MaybeTooltip.svelte';
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';

  export let id: string;
  export let event: string;
  export let name: string;
  export let description = '';

  $: descriptionHTML = markdown(description);

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer) {
      transfer.setData('kind', 'event')
      transfer.setData('id', id)
      if (e.ctrlKey) {
        transfer.effectAllowed = "copy";
      } else {
        transfer.effectAllowed = "move";
      }
    }
  }
</script>

<MaybeTooltip tip={descriptionHTML}>
  <div
    {id}
    draggable=true
    on:dragstart={handleDragStart}
    class="event m-[1.4375rem] w-[6.125rem] h-[6.125rem] p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-event-dark via-event to-event-light" >
    {name}
  </div>
</MaybeTooltip>
