<script lang="ts">
  import { goto } from '$app/navigation';
  import Grid from '$components/design/Grid.svelte';
  import type { LayoutData } from './$types';

  export let data: LayoutData;

  const { grid, decider } = data;
</script>

<slot />

<Grid
  {decider}
  default_audience_placements={$grid?.default_audience}
  audiences={$grid?.audiences}
  timeline_placements={$grid?.timeline}
  streams={$grid?.streams}
  default_stream_placements={$grid?.default_stream}
  flows={$grid?.flows}
  column_count={$grid?.column_count || 0} />
