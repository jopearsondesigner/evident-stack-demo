<script lang="ts">
  import classNames from 'classnames';
  export let href: string = '';
  export let active: boolean = false;
  export let activeClass: string='';
  export let nonActiveClass: string ='';
  export {liClass as class}
  let liClass:string;
  $: liClass = classNames(
    'text-sm text-body-light hover:text-body-dark dark:text-body-dark dark:hover:text-body-light whitespace-nowrap transition duration-200 ease-in-out inline-flex no-underline',
    active ? activeClass : nonActiveClass,
    $$props.class
  );
  let mobileLi='p-4 transition duration-200 ease-in-out w-full';
  let mobileNavChild='flex justify-between items-center w-full';
  export let hidden:boolean = false;
  export let footer:boolean = false;
</script>

{#if !hidden}
<li class={liClass}>
  <svelte:element
    this={href ? 'a' : 'div'}
    {href}
    {...$$restProps}
    on:blur
    on:change
    on:click
    on:focus
    on:keydown
    on:keypress
    on:keyup
    on:mouseenter
    on:mouseleave
    on:mouseover>
      <slot />
  </svelte:element>
</li>
{:else if footer}
<li class={classNames('flex justify-center', liClass, mobileLi)}>
  <svelte:element
    this={href ? 'a' : 'div'}
    {href}
    {...$$restProps}
    class={classNames()}
    on:blur
    on:change
    on:click
    on:focus
    on:keydown
    on:keypress
    on:keyup
    on:mouseenter
    on:mouseleave
    on:mouseover>
      <slot/>
  </svelte:element>
</li>
{:else}
<li class={classNames(liClass, mobileLi)}>
  <svelte:element
    this={href ? 'a' : 'div'}
    {href}
    {...$$restProps}
    class={classNames(mobileNavChild)}
    on:blur
    on:change
    on:click
    on:focus
    on:keydown
    on:keypress
    on:keyup
    on:mouseenter
    on:mouseleave
    on:mouseover>
      <slot/>
  </svelte:element>
</li>
{/if}
