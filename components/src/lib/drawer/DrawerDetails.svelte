<script lang="ts">
  import classNames from 'classnames';
  import IconButton from '../IconButton.svelte';
  import Icon from '../Icon.svelte';
  import ArrowUp from '../icons/ArrowUp.svelte';
  import ArrowDown from '../icons/ArrowDown.svelte';
  import MaybeTooltip from '../utils/MaybeTooltip.svelte';

  export let nameClass = '';
  export let descClass = '';
  export let divClass =
    'relative grid justify-items-stretch bg-black/[2%] dark:bg-black/[8%] border-b border-gray-primary dark:border-border-dark py-4';
  export let hClass =
    'text-sm font-bold text-body-light dark:text-gray-brand-4 text-center leading-[1.27] mx-[20px]';
  export let detailsClass = '';
  export let summaryClass = '';
  export let iconBtnClass =
    'absolute right-0.5 top-5 text-current rounded-full border-none p-0 flex items-center justify-center bg-gray-brand-1/0 dark:bg-white/0 hover:bg-gray-brand-1/[.09] dark:hover:bg-white/[.09] cursor-pointer';
  export let isClosed = true;
  export let name: string;
  export let description: string;
  // TODO: sync status indicator, per Dexie SyncStatus
  export let sync_status: number;

  let editable = false;

  function handleDblClick(event: any) {
    editable = true; // or use  editable=!editable  to toggle
  }

  export let isOpen: boolean;
  export const handleClick = () => (isOpen = !isOpen);
</script>

<div class={classNames(divClass)}>
  <h2 class={hClass} on:dblclick={handleDblClick} contenteditable="true" bind:textContent={name}>
    {name}
  </h2>
  <details bind:open={isOpen} class={detailsClass}>
    <summary class={summaryClass}>
      <div class={iconBtnClass}>
        <MaybeTooltip tip="Project Description" position="tooltip-bottom">
          {#if isOpen}
            <Icon
              name="close-up"
              size={12}
              class={classNames('mx-px flex-none stroke-1')}
              iconColor="text-gray-brand-1 dark:text-gray-brand-4"
              pathName={ArrowUp}
            />
          {:else}
            <Icon
              name="open-down"
              size={12}
              class={classNames('mx-px flex-none stroke-1')}
              iconColor="text-gray-brand-1 dark:text-gray-brand-4"
              pathName={ArrowDown}
            />
          {/if}
        </MaybeTooltip>
      </div>
    </summary>

    <p
      class={classNames(
        'font-medium text-default text-body dark:text-white leading-normal text-center mx-4 mt-2 max-h-16 overflow-auto',
        descClass
      )}
      contenteditable="true"
      placeholder="Project Description"
      bind:textContent={description}
    >
      {description}
    </p>
  </details>
</div>

<style>
  summary::-webkit-details-marker {
    display: none !important;
  }
  summary {
    list-style: none !important;
  }
  [contenteditable] {
    padding: 0.125em;
    border-radius: 4px;
    border: none;
  }
  [contenteditable]:focus {
    outline: 1px solid #1e6aff;
  }
</style>
