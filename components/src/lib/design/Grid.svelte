<script>
  import Audience from './grid/Audience.svelte';
  import Timeline from './grid/Timeline.svelte';
  import Stream   from './grid/Stream.svelte';

  // TODO: use Rust-generated types
  /* @type {Object.<number,
             {id:   string,
             type: 'interface',
             title: string,
             description?: string,
             }>} */
  export let default_audience;

  /* @type {Array.<{title: string,
             placements: {Object.<number,
             {id:   string,
             type: 'interface',
             title: string,
             description?: string,
             }>}}>} */
  export let audiences;

  // TODO: use Rust-generated types
  /* @type {Object.<number,
             {id:   string,
             type: ('command' | 'readModel'),
             title: string,
             description?: string,
             }>} */
  export let timeline;

  /* @type {Array.<{title: string,
             placements: {Object.<number,
             {id:   string,
             type: 'event',
             title: string,
             description?: string,
             }>}}>} */
  export let streams;

  // TODO: use Rust-generated types
  /* @type {Object.<number,
             {id:   string,
             type: 'event',
             title: string,
             description?: string,
             }>} */
  export let default_stream;

  $: timelineRow = 1 + audiences.length + 1

  $: defaultStreamRow = timelineOffset + streams.length + 1
</script>

<Audience row=1 placements={default_audience} />

{#each audiences as audience, i}
	<Audience row={i + 2} title="{audience.title}" placements={audience.placements}  />
{/each}

<Timeline row={timelineRow} placements={timeline} />

{#each streams as stream, i}
	<Stream row={i + 1 + timelineRow} title="{stream.title}" placements={stream.placements}  />
{/each}

<Stream row={defaultStreamRow} placements={default_stream} />
