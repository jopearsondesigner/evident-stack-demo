<script lang="ts">
  import type { CellType } from '../Grid';

  export let column: number;
  export let kind: CellType;
  export let lane: string | undefined = undefined;

</script>

<div
  on:dragover={(e) => {
    e.preventDefault();
  }}
  class="empty-cell min-w-placementPadded min-h-placementPadded"
/>
