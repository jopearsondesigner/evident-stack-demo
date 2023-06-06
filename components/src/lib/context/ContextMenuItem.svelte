<script lang="ts">
  // import { getContext } from 'svelte';
  // import { key } from './contextMenu.js';

  let liClass = 'px-1';
  export let childClass =
    'w-full text-left text-body dark:text-white cursor-default font-medium py-2 px-4 text-sm hover:bg-focus/20 hover:text-body dark:hover:text-white transition duration-200 ease-in';
  export let href: string | undefined = undefined;
  export let isDisabled = false;

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  // const { dispatchClick } = getContext(key);

  const handleClick = (_e: any) => {
    if (isDisabled) return;

    dispatch('click');
    // dispatchClick();
  };
</script>

<li class={liClass}>
  <svelte:element
    this={href ? 'a' : 'button'}
    {href}
    class:disabled={isDisabled}
    on:click={handleClick}
    class={childClass}
    {...$$restProps}
    on:click
    on:change
    on:keydown
    on:keyup
    on:focus
    on:blur
    on:mouseenter
    on:mouseleave>
    <slot />
  </svelte:element>
</li>
