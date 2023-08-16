<script>
  import classNames from 'classnames';
  export let label = '';
  export let featureData = false;
  export let headerText = false;
  export let headerItem = false;
  export let active = false;
  export let blank = true;
  export let headerTextClass = 'font-extrabold text-default text-body dark:text-white';
  export let featureTextClass = 'font-medium text-default text-body dark:text-white';
  export let sidebarItemClass = 'sidebarItem flex items-center';
  export let padding = 'pl-4 pr-1';
  export let divClass =
    'min-h-8 items-center grow overflow-auto bg-white dark:bg-dark-2 text-body-light dark:text-white w-full cursor-default';
  export let sidebarBtnClass =
    'bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] dark:text-white focus:text-body focus:bg-focus/[.20] dark:hover:bg-focus/[.20] transition duration-200 ease-in w-full space-x-3 h-8 cursor-default';
  export let sidebarActiveClass =
    'bg-focus hover:bg-focus/[100] text-white focus:text-white focus:bg-focus/[100] transition duration-200 ease-in w-full space-x-3 h-8 cursor-default';
  export let sidebarHeaderClass =
    'bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] dark:text-white transition duration-200 ease-in w-full space-x-3 h-8 cursor-default';
  export let sidebarSpanClass = 'flex-1 ml-3 text-left whitespace-nowrap';
</script>

<li class="sidebarItem">
  {#if featureData}
    <button class={classNames(sidebarItemClass, sidebarBtnClass)}>
      {#if headerText}
        <span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
      {:else}
        <span class={classNames(featureTextClass, sidebarSpanClass)}>{label}</span>
      {/if}
      <slot />
    </button>
  {:else if headerItem}
    <div class={classNames(sidebarItemClass, sidebarHeaderClass, padding)}>
      <slot name="leftIcon" />
      <span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
      <slot name="rightIcon" />
    </div>
  {:else if active}
    <button class={classNames(sidebarItemClass, sidebarActiveClass, padding)}>
      {#if headerText}
        <span style="color: white;" class={classNames(headerTextClass, sidebarSpanClass)}
          >{label}</span
        >
      {:else}
        <span style="color: white;" class={classNames(featureTextClass, sidebarSpanClass)}
          >{label}</span
        >
      {/if}
      <slot name="icon" />
    </button>
  {:else if blank}
    <div class={classNames(divClass, padding)}>
      <span class={classNames(featureTextClass)}>{label}</span>
      <slot />
    </div>
  {:else}
    <button class={classNames(sidebarItemClass, sidebarBtnClass, padding)}>
      {#if headerText}
        <span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
      {:else}
        <span class={classNames(featureTextClass, sidebarSpanClass)}>{label}</span>
      {/if}
      <slot name="icon" />
    </button>
  {/if}
</li>
