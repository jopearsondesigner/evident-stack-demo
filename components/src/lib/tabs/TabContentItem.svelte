<script lang="ts">
  import classNames from 'classnames';

  export let activeClass = '';
  export let inactiveClass = '';

  export let id: number;
  export let activeTabValue: number;
  export let divClass = '';
</script>

{#if activeTabValue === id}
  <div
    class={classNames('absolute', activeTabValue === id ? activeClass : inactiveClass, divClass)}
    id="tabitem-{id}"
    role="tabpanel"
    aria-labelledby="tab-{id}"
  >
    <slot />
  </div>
{/if}
