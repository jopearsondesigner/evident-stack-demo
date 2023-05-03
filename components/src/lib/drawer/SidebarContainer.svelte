<script>
  import classNames from 'classnames';
  import { slide } from 'svelte/transition';
  import { sineIn } from 'svelte/easing';

  export let brandClass =
    'inline bg-white dark:bg-dark-2 flex justify-center w-full cursor-default';
  export let btnClass =
    'pl-4 pr-1 h-8 transition duration-200 ease-in w-full bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] focus:text-body focus:bg-focus/[.20] dark:focus:bg-focus/[.20] cursor-default font-extrabold text-default text-body dark:text-white text-left whitespace-nowrap';

  export let title = 'Title';

  export let expanded = false;

  export let id = 'item' + Math.random().toString(36);
  export let alt = '';
  export let height = 28;
  export let src = '';

  /**
   * Obtain a reference to the `button` element
   * @type {HTMLButtonElement | null}
   */
  export let ref = null;

  import { getContext, onMount } from 'svelte';

  const ctx = getContext('Accordion');

  /**
   * @type {(() => void) | undefined}
   */
  let unsubscribe = undefined;

  onMount(() => {
    return () => {
      if (ctx) ctx.remove({ id });
      if (unsubscribe) unsubscribe();
      console.log(ctx);
    };
  });

  $: button_id = `button-${id}`;
  $: if (ctx) {
    ctx.add({ id, expanded });
    unsubscribe = ctx.items.subscribe((/** @type {{ [x: string]: boolean; }} */ value) => {
      expanded = value[id];
    });
  }

  export let disabled = false;
  export const isClosed = true;
</script>

<li data-accordion-item {...$$restProps} class:grow={expanded}>
  <button
    bind:this={ref}
    type="button"
    aria-expanded={expanded}
    aria-controls={id}
    aria-disabled={disabled}
    id={button_id}
    on:click
    disabled={expanded}
    class={expanded ? brandClass : btnClass}
    on:click={() => {
      if (ctx) {
        ctx.toggle({ id, expanded: !expanded });
        if (expanded && ref && ref.getBoundingClientRect().top < 0) {
          ref.scrollIntoView();
        }
      }
    }}
  >
    {#if expanded}
      <span class="py-6 px-1">
        <img {src} class={classNames()} {alt} {height} style="height:{height}px" />
      </span>
    {:else}
      <slot name="title">{title}</slot>
    {/if}
  </button>
  {#if expanded}
    <div
      role="region"
      {id}
      aria-labelledby={button_id}
      hidden={!expanded}
      class={classNames('h-full')}
      transition:slide={{ delay: 0, duration: 200, easing: sineIn }}
    >
      <slot />
    </div>
  {/if}
</li>
