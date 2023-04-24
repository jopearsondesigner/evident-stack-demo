<script lang="ts">
  import classNames from 'classnames';
  export let tip: string | undefined;
  let divClass = 'tooltip before:text-xs before:text-gray-primary';
  export let position = 'tooltip-top';
</script>

<!-- TODO: support HTML (i.e. Markdown output) -->

{#if tip}
  <div class={classNames(position, divClass)} data-tip={tip}>
    <slot />
  </div>
{:else}
  <slot />
{/if}
