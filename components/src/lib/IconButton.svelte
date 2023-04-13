<script lang="ts">
  export let href: string | undefined = undefined;
  export let name = '';
  export let iconBtnClass =
    'rounded-full border-none p-0 inline-flex items-center justify-center bg-gray-brand-1/0 dark:bg-white/0 hover:bg-gray-brand-1/[.09] dark:hover:bg-white/[.09] transition duration-200 ease-in cursor-default';
  export let size = 36;

  let hidden = false;
  const handleClick = () => {
    hidden = !hidden;
  };
</script>

<svelte:element
  this={href ? 'a' : 'button'}
  {href}
  {...$$restProps}
  class={iconBtnClass}
  style="width: {size}px; height: {size}px;"
>
  <slot />
</svelte:element>
