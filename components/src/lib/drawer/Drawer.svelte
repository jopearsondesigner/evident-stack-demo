<script>
  import classNames from 'classnames';
  import CloseButton from '../utils/CloseButton.svelte';
  import { fly } from 'svelte/transition';
  import { sineOut } from 'svelte/easing';
  import { clickOutside } from '../utils/clickOutside';
  export let activateClickOutside = true;
  export let hidden = true;
  export let divClass = 'z-40 top-0 fixed h-screen bg-white dark:bg-dark-2';
  export let className = '';
  export let leftOffset = 'left-0';
  export let rightOffset = 'right-0';
  export let placement = 'left';
  export let navbarHeight = 64;
  export let name = '';
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
  export let drawerRight = false;

  const placements = {
    left: leftOffset,
    right: rightOffset
  };

  export let backdropClasses = 'bg-gray-900 bg-opacity-50 dark:bg-opacity-80';
</script>

{#if !hidden}
  {#if !drawerRight}
    <div
      transition:fly={transitionParams}
      class={classNames(className, divClass)}
      style="padding-top: {navbarHeight}px;"
    >
      <slot />
    </div>
  {:else if activateClickOutside}
    <div class={classNames('fixed inset-0 z-30', backdropClasses)} />
    <div
      use:clickOutside={() => !hidden && handleDrawer()}
      transition:fly={transitionParamsRight}
      class={classNames(className, divClass, placements[placement])}
      style="padding-top: {navbarHeight}px;"
    >
      <CloseButton
        {name}
        size={12}
        btnClass="float-right mt-2 mr-2"
        on:click={() => (hidden = true)}
        color={$$restProps.color}
      />
      <slot />
    </div>
  {/if}
{/if}
