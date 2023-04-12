<script lang="ts">
  import classNames from 'classnames';
  export let navClass =
    'flex items-center justify-end text-sm text-body-light hover:text-[1.125rem] hover:text-white dark:text-white w-full border border-border-light dark:border-border-dark p-3 indexNavShadow bg-left-top bg-no-repeat bg-contain';
  export let hoverClass =
    'w-full h-auto bg-gradient-radial from-transparent to-transparent hover:from-[#525C6F]/[.3] hover:to-[#525C6F]/[.5]';
  export let rounded: string = '';
  export let backgroundImg = 'bg-[url("$lib/assets/images/product/design/modelThumb.svg")]';
  export let btnClass: string = 'rounded-lg';
  export let linkClass: string = '';
  export let imgClass: string = '';
  export let href: string | undefined = undefined;
  export let img: string | undefined = undefined;
  export let height: number;
</script>

<div class={classNames(hoverClass, rounded, $$props.class)}>
  <svelte:element
    this={href ? 'a' : 'button'}
    {href}
    class={classNames(href ? linkClass + backgroundImg : btnClass, navClass)}
    style="height: {height}px;"
    {...$$restProps}
    on:click
    on:mouseover
    on:mouseout
    on:focus
    on:blur
  >
    {#if img}
      <img class={imgClass} src={img} alt="" />
      <slot />
    {:else}
      <slot />
    {/if}
  </svelte:element>
</div>

<style>
  .indexNavShadow {
    box-shadow: 0px 2px 2px 0px rgba(0, 0, 0, 0.16);
  }
</style>
