<script>
  import Event from './Event.svelte';

  /* @type {number} */
  export let row = 0;

  /* @type {string} */
  export let title = "";

  /* @type {Object.<number,
             {id:   string,
             type: 'event',
             title: string,
             description?: string,
             index: number}>} */
  export let placements = {};
</script>

{#if title}
  <h3 class="stream laneTitle" style="grid-row: {row} / {row};">{title}</h3>
{/if}

<div class="stream" style="grid-row: {row} / {row};" />

{#each Object.values(placements) as {id, title, description, index} (id) }
  <Event id={id} title={title} description={description} row={row} column={index + 1} />
{/each}
