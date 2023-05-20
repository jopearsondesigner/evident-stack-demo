<script lang="ts">
  import classNames from 'classnames';
  import type { SizeType } from './types';
  import Frame from './utils/Frame.svelte';
  import { createEventDispatcher } from 'svelte';
  import CloseButton from './utils/CloseButton.svelte';
  import focusTrap from './utils/focusTrap';
  export let open = false;
  export let title = '';
  export let size: SizeType = 'md';
  export let placement = 'center';
  export let autoclose = false;
  export let permanent = false;
  export let backdropClasses = 'bg-gray-900 bg-opacity-50 dark:bg-opacity-80';
  const dispatch = createEventDispatcher();

  const handleClose = (e: { target?: any; preventDefault?: any }) => {
    e.preventDefault();
    dispatch('close')
  };

  function prepareFocus(node: HTMLElement) {
    const walker = document.createTreeWalker(node, NodeFilter.SHOW_ELEMENT);
    let n: Node | null;
    while ((n = walker.nextNode())) {
      if (n instanceof HTMLElement) {
        const el = n as HTMLElement;
        const [x, y] = isScrollable(el);
        if (x || y) el.tabIndex = 0;
      }
    }
    node.focus();
  }

  const getPlacementClasses = () => {
    switch (placement) {
      // top
      case 'top-left':
        return ['justify-start', 'items-start'];
      case 'top-center':
        return ['justify-center', 'items-start'];
      case 'top-right':
        return ['justify-end', 'items-start'];
      // center
      case 'center-left':
        return ['justify-start', 'items-center'];
      case 'center':
        return ['justify-center', 'items-center'];
      case 'center-right':
        return ['justify-end', 'items-center'];
      // bottom
      case 'bottom-left':
        return ['justify-start', 'items-end'];
      case 'bottom-center':
        return ['justify-center', 'items-end'];
      case 'bottom-right':
        return ['justify-end', 'items-end'];
      default:
        return ['justify-center', 'items-center'];
    }
  };
  const sizes = {
    xs: 'max-w-md',
    sm: 'max-w-lg',
    md: 'max-w-2xl',
    lg: 'max-w-4xl',
    xl: 'max-w-7xl'
  };
  const onAutoClose = (e: { target: any }) => {
    const target = e.target;
    if (autoclose && target?.tagName === 'BUTTON') handleClose(e);
  };

  let frameClass: string;
  $: frameClass = classNames(
    'relative flex flex-col mx-auto border border-border-light dark:border-border-dark',
    $$props.class
  );
  const isScrollable = (e: Element) => [
    e.scrollWidth > e.clientWidth && ['scroll', 'auto'].indexOf(getComputedStyle(e).overflowX) >= 0,
    e.scrollHeight > e.clientHeight &&
      ['scroll', 'auto'].indexOf(getComputedStyle(e).overflowY) >= 0
  ];
  function preventWheelDefault(e: { preventDefault: () => any }) {
    // @ts-ignore
    const [x, y] = isScrollable(this);
    return x || y || e.preventDefault();
  }
  function handleKeys(e: { key?: any; target?: any; preventDefault?: any }) {
    if (e.key === 'Escape' && !permanent) return handleClose(e);
  }
</script>

{#if open}
  <!-- backdrop -->
  <div class={classNames('fixed inset-0 z-40', backdropClasses)} />
  <!-- dialog -->
  <div
    on:keydown={handleKeys}
    on:wheel|preventDefault
    use:prepareFocus
    use:focusTrap
    on:click={autoclose ? onAutoClose : null}
    class={classNames(
      'fixed top-0 left-0 right-0 h-modal md:inset-0 md:h-full z-50 w-[]375px] p-4 flex',
      ...getPlacementClasses()
    )}
    tabindex="-1"
    aria-modal="true"
    role="dialog">
    <div class="flex relative {sizes[size]} w-full max-h-full">
      <!-- Modal content -->
      <Frame shadow {...$$restProps} class={frameClass}>
        <!-- Modal header -->
        {#if $$slots.header || title}
          <Frame
            color={$$restProps.color}
            class="flex justify-between items-center p-3 border-b border-border-gray-secondary dark:border-border-dark">
            <slot name="header">
              <h3
                class="text-default w-full text-center font-extrabold {$$restProps.color}
                  ? ''
                  : 'text-body-light dark:text-body-dark'} p-0"
              >
                {title}
              </h3>
            </slot>
            {#if !permanent}<CloseButton name="Close modal" size={12} on:click={handleClose} />{/if}
          </Frame>
        {:else if !permanent}
          <CloseButton
            name="Close modal"
            size={12}
            btnClass="absolute top-3 right-2.5"
            on:click={handleClose} />
        {/if}
        <!-- Modal body -->
        <div
          id="modal"
          class="	p-6 space-y-3 flex-1 overflow-y-auto overscroll-contain min-w-[375px]"
          on:keydown|stopPropagation={handleKeys}
          on:wheel|stopPropagation={preventWheelDefault}
        >
          <slot />
        </div>
        <!-- Modal footer -->
        {#if $$slots.footer}
          <Frame
            color={$$restProps.color}
            class="flex items-end justify-end px-6 pt-3 pb-6 space-x-3"
          >
            <slot name="footer" />
          </Frame>
        {/if}
      </Frame>
    </div>
  </div>
{/if}
