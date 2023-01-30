<script>
  import Command from './Command.svelte';
  import ReadModel from './ReadModel.svelte';

  /* @type {number} */
  export let row = 0;

  /* @type {Object.<number,
             {id:   string,
             type: ('command', 'readModel'),
             title: string,
             description?: string,
             index: number
             }>} */
  export let placements = {};
</script>

<h3 class="timeline laneTitle" style="grid-row: {row} / {row};">Timeline</h3>

<div style="grid-row: {row} / {row};" />

{#each Object.values(placements) as {id, type, title, description, index} (id) }
  {#if type === 'command'}
    <Command id={id} title={title} description={description} row={row} column={index + 1} />
  {:else if type === 'readModel'}
    <ReadModel id={id} title={title} description={description} row={row} column={index + 1} />
  {/if}
{/each}
