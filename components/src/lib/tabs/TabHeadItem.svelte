<script lang="ts">
  import classNames from 'classnames';
  import { page } from '$app/stores';
  import { createEventDispatcher } from 'svelte';
  import CloseButton from '../utils/CloseButton.svelte';
  export let id: number;
  export let activeTabValue: number = 0;
  export let inactiveClass: string =
    'bg-dark-2 hover:bg-[#303841]/[.40] text-brand-gray-4 hover:text-white';
  export let activeClass: string = 'active bg-[#303841] text-white';
  export let liClass: string = '';
  export let btnClass: string =
    'transition duration-200 ease-in h-[28px] max-w-[188px] w-auto whitespace-nowrap rounded-t-[6px] pl-[12px] !m-0 relative bottom-0 h-[28px]';
  export let nameClass: string = 'text-default text-left font-medium text-inherit mr-3';
  export let name: string = 'Tab name';
  let className: string = '';
  export { className as class };

  const dispatch = createEventDispatcher();

  const handleClose = (e: { target?: any; preventDefault?: any }) => {
    e.preventDefault();
    dispatch('close');
  };

  export let type: 'submit' | 'reset' | 'button' | undefined | null = 'button';
  export let tabindex: number = 0;
  let iconColor = 'text-white';
  export let href = '';
</script>

<li class={liClass} role="presentation">
  <svelte:element
    this={href ? 'a' : 'button'}
    {href}
    on:click
    {tabindex}
    class={classNames(
      activeTabValue === id ? activeClass : inactiveClass,
      btnClass,
      className,
      'lg:mr-2.5 flex items-center justify-between'
    )}
    class:selected={$page.url.pathname === href}
    id="tab-{id}"
    role="tab"
  >
    <div class={nameClass}>{name}</div>
    <div class="flex">
      <CloseButton
        type="button"
        name="close-button"
        size={8}
        on:click={handleClose}
        btnClass="p-1 mr-1.5"
        {iconColor}
      />

      <div
        class="divider border-r border-gray-brand-1 max-h-[16px] -mr-[1px] my-auto ml-0 w-px z-[1] relative"
      />
    </div>
  </svelte:element>
</li>

<style>
  .active,
  .selected {
    z-index: 2;
  }
  .active .divider,
  .selected .divider {
    display: none;
  }
  .selected {
    background-color: #303841 !important;
    color: white !important;
  }
</style>
