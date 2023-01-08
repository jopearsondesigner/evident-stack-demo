<script lang="ts">
    // import type { SizeType } from '$lib/types';
    import classNames from 'classnames';
    export let imgClass:string = '';
    export let cardClass:string = '';
    export let href: string | undefined = undefined;
    export let img: string | undefined = undefined;
    // export const size: SizeType = 'sm';

    const sizes = {
        xs: 'max-w-xs',
        sm: 'max-w-sm',
        md: 'max-w-lg',
        lg: 'max-w-2xl',
        xl: 'max-w-screen-xl'
    };
  </script>

<svelte:element this={href ? 'a' : 'div'}
    {href}
    {...$$restProps}>
    {#if img}
      <img class={imgClass} src={img} alt="" />
      <div class={cardClass}>
        <slot />
      </div>
    {:else}
      <slot />
    {/if}
</svelte:element>


