<script>
  import MaybeTooltip from '../../utils/MaybeTooltip.svelte';
  import markdown from '../../utils/markdown.js';

  /* TODO: UUID */
  /* @type {string} */
  export let id = "";

  /* @type {string} */
  export let title = "";

  /* @type {string | null} */
  export let description = "";

  /* @type {number} */
  export let row = 0;

  /* @type {number} */
  export let column = 0;

  /* @type {boolean} */
  export let cursor = false;

  $: descriptionHTML = markdown(description);
  $: gridRow = row + 1;
  $: gridColumn = column + 1;
</script>

<div
  id="{id}"
  class="placement command z-30 flex place-self-center align-items-center p-3"
  class:cursor
  style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};">
  <MaybeTooltip tip="{descriptionHTML}">
    <div class="command w-32 h-32 p-3 overflow-visible text-left leading-tight shadow-placement bg-gradient-to-b from-command to-command-light">
      {title}
    </div>
  </MaybeTooltip>
</div>
