<script>
  export let tip;
</script>

<!-- TODO: support HTML (i.e. Markdown output) -->

{#if tip}
  <div class="tooltip" data-tip="{tip}"><slot/></div>
{:else}
  <slot/>
{/if}
