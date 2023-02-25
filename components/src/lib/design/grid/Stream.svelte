<script>
  /** @type {number} */
  export let row = 0;
  $: gridRow = row + 1;

  /** @type {string} */
  export let title = "";

</script>

{#if title}
  <h3 class="streamTitle laneTitle sticky left-3 z-30 justify-self-start self-end cursor-pointer"
      style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};">
    {title}
  </h3>
{:else}
  <!-- TODO: reduce the color to disabled text -->
  <h3 class="streamTitle sticky left-3 z-30 justify-self-start self-end"
      style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};">
    Default Stream
  </h3>
{/if}

<div class="stream absolute top-0 -left-3 bottom-0 -right-6 border-b-2"
     style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};" />

<slot />
