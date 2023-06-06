<script lang="ts">
  import classNames from 'classnames';
  import { setContext, createEventDispatcher } from 'svelte';
  import { fade } from 'svelte/transition';
  import { key } from './contextMenu';

  export let divClass: string =
  'absolute grid overflow-hidden whitespace-nowrap z-40 transform w-auto rounded-lg shadow-xl border border-light dark:border-border-dark bg-white dark:bg-dark-2 py-3';

  export let x: number;
  export let y: number;
  let menuEl: Element;

  // whenever x and y is changed, restrict box to be within bounds
  $: if (menuEl) {
    const rect = menuEl.getBoundingClientRect();
    x = Math.min(window.innerWidth - rect.width, x);
    if (y > window.innerHeight - rect.height) y -= rect.height;
  };

  const dispatch = createEventDispatcher();

  setContext(key, {
    dispatchClick: () => dispatch('click')
  });

  const onPageClick = (e: MouseEvent) => {
    if (e.target === menuEl || menuEl.contains(e.target as Node)) return;
    dispatch('clickoutside');
  }
</script>

<svelte:body on:click={onPageClick} />

<ul
  class={classNames(divClass)}
  transition:fade={{ duration: 100 }}
  bind:this={menuEl}
  style="top: {y}px; left: {x}px;">
  <slot />
</ul>
