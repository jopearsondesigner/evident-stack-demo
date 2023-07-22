<script lang="ts">
  import classNames from 'classnames';
  export let navDivClass =
    'xl:container flex justify-between items-center lg:py-0 lg:justify-start lg:space-x-8';
  export let mobileNavDiv =
    'lg:hidden m-2 absolute top-0 inset-x-0 rounded-lg shadow-lg bg-white dark:bg-dark-2 border border-border-light dark:border-border-dark';
  export let hidden: boolean = false;
</script>

{#if !hidden}
  <div class={navDivClass}>
    <slot />
  </div>
{:else}
  <div class={classNames(mobileNavDiv)}>
    <slot />
  </div>
{/if}
