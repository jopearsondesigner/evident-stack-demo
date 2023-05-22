<script lang="ts">
  import classNames from 'classnames';
  import CloseButton from '../utils/CloseButton.svelte';
  import { fly } from 'svelte/transition';
  import { sineOut } from 'svelte/easing';
  import { clickOutside } from '../utils/clickOutside';
  import { createEventDispatcher } from 'svelte';
  import { createKeybindingsHandler, type KeyBindingMap } from '../vendor/tinykeys/tinykeys';
  export let activateClickOutside: boolean = true;
  export let hidden: boolean = true;
  export let divClass: string = 'z-40 top-0 fixed h-full bg-white dark:bg-dark-2';
  let className: string = '';
  export {className as class};
  export let leftOffset: string = 'left-0';
  export let rightOffset: string = 'right-0';
  export let placement: 'left' | 'right';
  export let navbarHeight: number = 64;
  export let name: string = '';
  let transitionParams = {
    x: -240,
    duration: 200,
    easing: sineOut
  };
  let transitionParamsRight = {
    x: 480,
    duration: 200,
    easing: sineOut
  };
  export let drawerRight = placement == 'right';

  const placements = {
    left: leftOffset,
    right: rightOffset
  };

  export let backdropClasses = 'bg-gray-900 bg-opacity-50 dark:bg-opacity-80';

  const dispatch = createEventDispatcher()

  export const handleClose = () => {
    dispatch('close')
  }

  const drawerKeys: KeyBindingMap = {
    Escape: handleClose
  };

  const keyboardHandler: EventListener = createKeybindingsHandler(drawerKeys);
</script>

<svelte:window on:keydown={keyboardHandler} />

{#if !hidden}
  {#if !drawerRight}
    <div
      transition:fly={transitionParams}
      class={classNames(className, divClass)}
      style="padding-top: {navbarHeight}px;">
      <slot />
    </div>
  {:else if activateClickOutside}
    <div class={classNames('fixed inset-0 z-30', backdropClasses)} />
    <div
      use:clickOutside={handleClose}
      transition:fly={transitionParamsRight}
      class={classNames(className, divClass, placements[placement])}
      style="padding-top: {navbarHeight}px;" >
      <CloseButton
        {name}
        size={12}
        btnClass="float-right mt-2 mr-2"
        on:click={handleClose} />
      <slot />
    </div>
  {/if}
{/if}
