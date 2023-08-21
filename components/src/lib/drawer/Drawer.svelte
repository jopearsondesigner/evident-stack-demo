<script lang="ts">
  import classNames from 'classnames';
  import { onMount } from 'svelte';
  import CloseButton from '../utils/CloseButton.svelte';
  import { fly } from 'svelte/transition';
  import { sineOut } from 'svelte/easing';
  import { clickOutside } from '../utils/clickOutside';
  import { createEventDispatcher } from 'svelte';
  import { createKeybindingsHandler, type KeyBindingMap } from '../vendor/tinykeys/tinykeys';
  export let hidden: boolean = true;
  export let divClass: string =
    'z-40 top-0 bottom-0 box-border fixed flex flex-col bg-white dark:bg-dark-2';
  let className: string = '';
  export { className as class };
  export let leftOffset: string = 'left-0';
  export let rightOffset: string = 'right-0';
  export let placement: 'left' | 'right';
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

  const dispatch = createEventDispatcher();

  export const handleClose = () => {
    dispatch('close');
  };

  const drawerKeys: KeyBindingMap = {
    Escape: handleClose
  };

  const keyboardHandler: EventListener = createKeybindingsHandler(drawerKeys);

  export let win = window,
    doc = document,
    docElem = doc.documentElement,
    body = doc.getElementsByTagName('body')[0],
    height = win.innerHeight || docElem.clientHeight || body.clientHeight;
</script>

<svelte:window on:keydown={keyboardHandler} />

{#if !hidden}
  {#if drawerRight}
    <div
      class={classNames('fixed inset-0 z-30', backdropClasses)}
      on:click={handleClose}
      on:keypress={handleClose}
    />
    <slot name="extra" />
    <div
      transition:fly={transitionParamsRight}
      class={classNames(className, divClass, placements[placement])}
    >
      <CloseButton
        {name}
        size={12}
        type="button"
        btnClass="float-right mt-2 mr-2"
        on:click={handleClose}
      />
      <slot />
    </div>
  {:else}
    <div
      id="left-drawer"
      transition:fly={transitionParams}
      class={classNames(className, divClass)}
      style="max-height: {height}px !important"
    >
      <slot />
    </div>
  {/if}
{/if}
