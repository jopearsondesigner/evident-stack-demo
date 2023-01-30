<script>
  import Interface from './Interface.svelte';

  /* @type {number} */
  export let row = 0;

  /* @type {string} */
  export let title = "";

  /* @type {Object.<number,
             {id:   string,
             type: 'interface',
             title: string,
             description?: string,
             index: number
             }>} */
  export let placements = {};
</script>

{#if title}
  <h3 class="audience laneTitle" style="grid-row: {row} / {row};">{title}</h3>
{/if}

<div class="audience" style="grid-row: {row} / {row};" />

{#each Object.values(placements) as {id, title, description, index} (id) }
  <Interface id={id} title={title} description={description} row={row} column={index + 1} />
{/each}
