<script lang="ts">
  import MaybeTooltip from '../../utils/MaybeTooltip.svelte';
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';

  export let id: string;
  export let command: string;
  export let name: string;
  export let description = '';

  $: descriptionHTML = markdown(description);

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer) {
      transfer.setData('timeline', id);
      if (e.ctrlKey) {
        transfer.effectAllowed = 'copy';
      } else {
        transfer.effectAllowed = 'move';
      }
    }
  };
</script>

<MaybeTooltip tip={descriptionHTML}>
  <div
    {id}
    draggable="true"
    on:dragstart={handleDragStart}
    class="command m-[1.4375rem] w-[6.125rem] h-[6.125rem] p-2 overflow-visible text-left text-node font-semibold leading-tight bg-gradient-to-b from-command-dark via-command to-command-light"
  >
    {name}
  </div>
</MaybeTooltip>
