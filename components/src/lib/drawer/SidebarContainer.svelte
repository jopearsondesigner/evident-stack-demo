<script lang="ts">
  import classNames from 'classnames';
  import { slide } from 'svelte/transition';
  import { sineIn } from 'svelte/easing';
  import { goto } from '$app/navigation';

  export let expanded = false;

  export let brandClass =
    'inline bg-white dark:bg-dark-2 flex justify-center w-full cursor-default';
  export let btnClass =
    'pl-4 pr-1 h-8 uppercase transition duration-200 ease-in w-full bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] focus:text-body focus:bg-focus/[.20] dark:focus:bg-focus/[.20] cursor-default font-extrabold text-default text-body dark:text-gray-brand-4 text-left whitespace-nowrap';

  export let title: string;
  export let id: string;
  export let href: string;
  export let alt = '';
  export let height = 28;
  export let src = '';

  export let ref: HTMLButtonElement | null = null;

  $: button_id = `button-${id}`;

  export let disabled = false;
  export const isClosed = true;
</script>

<li
  data-accordion-item
  {...$$restProps}
  class:grow={expanded}
  class:border-b={!expanded}
  class="flex flex-col border-border-light dark:border-border-dark"
>
  <button
    bind:this={ref}
    type="button"
    aria-expanded={expanded}
    aria-controls={id}
    aria-disabled={disabled}
    id={button_id}
    on:click={() => goto(href)}
    on:click
    class={expanded ? brandClass : btnClass}
    disabled={expanded}
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
      class={classNames('grow')}
      in:slide={{ delay: 0, duration: 200, easing: sineIn }}
      out:slide={{ delay: 0, duration: 200, easing: sineIn }}
    >
      <slot />
    </div>
  {/if}
</li>
