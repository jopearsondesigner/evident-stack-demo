<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import Grid from '$components/design/Grid.svelte';
  import type { LayoutData } from './$types';
  import { debug } from '$lib/util';

  export let data: LayoutData;

  const { grid, decider } = data;

  const handleNavigateToPlacementDetails = (e: CustomEvent) => {
    debug("handling navigateToPlacementDetails", e, `/projects/${$page.params.id}/design/placements/${e.detail.placement}`);
    goto(`/projects/${$page.params.id}/design/placements/${e.detail.placement}`, {noScroll: true})
  }

  const handleNavigateToImportJson = (e: CustomEvent) => {
    debug("handling navigateToPlacementDetails", e, `/projects/${$page.params.id}/design/placements/import?column=${e.detail.column}`);
    goto(`/projects/${$page.params.id}/design/placements/import?column=${e.detail.column}`, {noScroll: true})
  }
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
  column_count={$grid?.column_count || 0}
  on:navigateToPlacementDetails={handleNavigateToPlacementDetails}
  on:navigateToImportJson={handleNavigateToImportJson}
  />
