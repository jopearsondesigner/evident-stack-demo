<script lang="ts">
  import classNames from 'classnames';

  export let linkClass: string =
    'text-left block font-medium mx-1 text-default text-body dark:text-white bg-white dark:bg-dark-2 hover:bg-focus/20 dark:hover:bg-focus/20 transition duration-200 ease-in cursor-default';
  export let divClass: string =
    'font-extrabold mx-1 text-default text-body dark:text-white transition duration-200 ease-in cursor-default';
  export let padding = 'py-2 px-4';
  export let textOnly: boolean = false;
  export let href: string | undefined = undefined;
  export let active: boolean = false;
  export let className: string = '';
</script>

{#if textOnly}
  <svelte:element
    this={href ? 'a' : 'button'}
    {href}
    class={classNames(linkClass, padding, className)}
    {...$$restProps}
  >
    <slot />
  </svelte:element>
{:else}
  <div class={classNames(divClass, padding, className)}>
    <slot />
  </div>
{/if}

<style>
  .active {
    background: #1e6aff !important;
    color: white !important;
    cursor: default;
  }
</style>
