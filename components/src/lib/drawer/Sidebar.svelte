<script lang="ts">
  import classNames from 'classnames';

  export let asideClass =
    'h-full flex-column bg-white dark:bg-dark-2 border-r border-gray-primary dark:border-gray-brand-3 transition-width duration-200 ease-in';
  export let nameClass =
    'text-md font-bold text-body-light dark:text-body-dark text-center leading-[1.27] mx-4 mb-2';
  export let descClass =
    'font-medium text-default text-body dark:text-white leading-normal text-center mx-4 mt-2 max-h-16 overflow-auto';
  export let divClass = 'grow-0 border-b border-gray-primary py-4';
  export let style = '';
  let className = '';
  export { className as class };
  export let isClosed = true;
  export let name: string;
  export let description: string;
  // TODO: sync status indicator, per Dexie SyncStatus
  export let sync_status: number;

  let editable = false;

  function handleDblClick(event: any) {
    editable = true; // or use  editable=!editable  to toggle
  }
</script>

<aside class={classNames(className, asideClass)} {style}>
  <div class={divClass}>
    <h2
      class={nameClass}
      on:dblclick={handleDblClick}
      contenteditable="true"
      bind:textContent={name}>
      {name}
    </h2>
    <p class={descClass} contenteditable="true" bind:textContent={description}>
      {description}
    </p>
  </div>
  <slot />
</aside>

<style>
  [contenteditable] {
    padding: 0.125em;
    border-radius: 4px;
    border: none;
  }
  [contenteditable]:focus {
    outline: 1px solid #1e6aff;
  }
</style>
