<script>
  import MaybeTooltip from '../../../utils/MaybeTooltip.svelte';
  import markdown from '../../../utils/markdown.js';

  /* TODO: UUID */
  /* @type {string} */
  export let id;

  /* @type {string} */
  export let title;

  /* @type {string | null} */
  export let description;

  /* @type {number} */
  export let row;

  /* @type {number} */
  export let column;

  /* @type {boolean} */
  export let cursor = false;

  $: descriptionHTML = markdown(description);
</script>

<div
  id={id}
  class="placement z-30 flex place-self-center align-items-center p-3"
  class:cursor
  style="grid-row: {row} / {row}; grid-column: {column} / {column};">
  <MaybeTooltip tip="{descriptionHTML}">
    <div class="event w-32 h-32 p-3 overflow-visible text-left leading-tight shadow-placement bg-gradient-to-b from-event to-event-light">
      {title}
    </div>
  </MaybeTooltip>
</div>
