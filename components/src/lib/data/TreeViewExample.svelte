<script lang="ts">
  import classNames from 'classnames';
  import TreeView, { type TreeItem } from './TreeView.svelte';

  let isActive: string | number;
  let btnClass: string | undefined = '';
  let summaryClass: string | undefined = '';
  let num: number = Math.floor(Math.random() * 100);

  const tree_data = [
    {
      name: 'Autonomo Mobile iOS App',
      type: 'event-model',
      id: { num },
      children: [
        {
          name: 'My Vehicles',
          type: 'read-model',
          id: { num },
          children: [
            {
              name: 'Vehicle Added',
              type: 'event',
              id: { num }
            },
            {
              name: 'Placement',
              type: 'placement',
              id: { num }
            },
            {
              name: 'Change',
              type: 'event',
              id: { num }
            },
            {
              name: 'Placement',
              type: 'placement',
              id: { num }
            },
            {
              name: 'Vehicle Removed',
              type: 'event',
              id: { num }
            },
            {
              name: 'Placement',
              type: 'placement',
              id: { num }
            },
            {
              name: 'Add a Vehicle',
              type: 'interface',
              id: { num }
            },
            {
              name: 'Placement',
              type: 'placement',
              id: { num }
            },
            {
              name: 'My Vehicles',
              type: 'interface',
              id: { num }
            },
            {
              name: 'Placement',
              type: 'placement',
              id: { num }
            },
            {
              name: 'Remove a Vehicle',
              type: 'interface',
              id: { num }
            },
            {
              name: 'Placement',
              type: 'placement',
              id: { num }
            }
          ]
        },
        {
          name: 'Add a Vehicle',
          type: 'command',
          id: { num },
          children: [
            {
              name: 'Vehicle Added',
              type: 'event',
              id: { num }
            },
            {
              name: 'Placement',
              type: 'placement',
              id: { num }
            },
            {
              name: 'Add a Vehicle',
              type: 'interface',
              id: { num }
            },
            {
              name: 'Placement',
              type: 'placement',
              id: { num }
            }
          ]
        }
      ]
    }
  ];

  import ThemeSwitch from '$lib/utils/ThemeSwitch.svelte';
</script>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-4"><ThemeSwitch /></span>

<div class="w-[240px] bg-white dark:bg-dark-2 h-screen overflow-hidden">
  <TreeView {tree_data} isClosed let:item>
    <div class="flex w-full group h-7">
      {#if item.children}
        <div class={classNames(summaryClass)}>
          <img
            src="../../../src/lib/assets/images/icons/{item.type}.svg"
            alt={item.type}
            width="14"
            class="inline-flex w-3.5 h-3.5 mr-0.5"
          />
          <span class="ml-1">{item.name}</span>
        </div>
      {:else}
        <button
          class={classNames(
            btnClass,
            'flex w-full pl-[35px] h-7 text-body dark:text-body-dark bg-transparent hover:bg-focus/[.20] transition duration-200 ease-in'
          )}
          class:selected={isActive === item.id}
          on:click={() => (isActive = item.id)}
        >
          {#if item.type == 'placement'}
            <span
              class="relative border-l border-b border-gray-brand-4 dark:border-gray-brand-1 w-1.5 h-5 -top-[9px] -right-1.5 mr-2"
            />
          {/if}
          {#if item.id == isActive && item.type == 'placement'}
            <img
              src="../../../src/lib/assets/images/icons/{item.type}-selected.svg"
              alt={item.type}
              width="14"
              class="inline-flex w-3.5 h-3.5"
            />
          {:else}
            <img
              src="../../../src/lib/assets/images/icons/{item.type}.svg"
              alt={item.type}
              width="14"
              class="inline-flex w-3.5 h-3.5"
            />
          {/if}
          <span class="ml-1">{item.name}</span>
        </button>
      {/if}
    </div>
  </TreeView>
</div>

<style>
  .selected {
    background-color: #1e6aff !important;
    color: white;
  }
</style>
