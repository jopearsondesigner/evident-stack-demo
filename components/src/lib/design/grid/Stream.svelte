<script>
  /* @type {number} */
  export let row = 0;

  /* @type {string} */
  export let title = "";
</script>

{#if title}
  <h3 class="stream laneTitle" style="grid-row: {row} / {row};">{title}</h3>
{/if}

<div class="stream" style="grid-row: {row} / {row};" />

<slot />
