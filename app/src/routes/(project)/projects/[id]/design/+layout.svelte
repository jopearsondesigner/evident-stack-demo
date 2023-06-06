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
    debug("handling navigateToPlacementDetails", e, `/projects/${$page.params.id}/design/import?column=${e.detail.column}`);
    goto(`/projects/${$page.params.id}/design/import?column=${e.detail.column}`, {noScroll: true})
  }
</script>

<slot />

{#if decider && $grid}
  <Grid
    mode={$page.data.mode}
    {decider}
    grid={$grid}
    on:navigateToPlacementDetails={handleNavigateToPlacementDetails}
    on:navigateToImportJson={handleNavigateToImportJson} />
  {/if}
