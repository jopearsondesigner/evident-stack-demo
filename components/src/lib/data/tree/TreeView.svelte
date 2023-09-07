<script lang="ts">
  import classNames from 'classnames';
  import type { TreeData } from './TreeItem.svelte';
  import Icon from '../../Icon.svelte';
  import IconButton from '../../IconButton.svelte';
  import CloseUp from '../../icons/CloseUp.svelte';
  import OpenDown from '../../icons/OpenDown.svelte';
  import { page } from '$app/stores';

  export let tree_data: TreeData = [];

  function summaryKeyup(event: KeyboardEvent) {
    // @ts-ignore
    if (event.key == ' ' && document.activeElement.tagName != 'SUMMARY') {
      event.preventDefault();
    }
  }

  export let isClosed: any | boolean | never[] = [];
  let summaryClass: string | undefined =
    'focus:pointer-events-none hover:pointer-events-auto grow text-body dark:text-white whitespace-nowrap text-ellipsis leading-normal flex items-center bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20]';
  let iconBtnClass =
    'text-current rounded-full border-none p-0 inline-flex items-center justify-center bg-gray-brand-1/0 dark:bg-white/0 hover:bg-gray-brand-1/[.09] dark:hover:bg-white/[.09] cursor-pointer';
  let margin = 'm-px';
  export let ulClass = '';
  export let spanClass = 'pointer-events-auto block h-7 w-auto flex justify-center items-center';
  export let isActive: string | number;
  export let href = '';
</script>

<ul class={ulClass}>
  {#each tree_data as item (item.id)}
    <li>
      {#if item.children}
        <details open>
          <summary
            class={classNames(summaryClass)}
            id={item.id}
            on:keyup={summaryKeyup}
            tabindex="0"
          >
            <span class={spanClass} class:selected={$page.route.id === item.id}>
              <IconButton
                bind:iconBtnClass
                {margin}
                size={20}
                on:click={() => (isClosed[item.id] = !isClosed[item.id])}
              >
                {#if !isClosed[item.id]}
                  <Icon
                    name="close-up"
                    size={14}
                    class="mx-1 flex-none"
                    iconColor="fill-current"
                    pathName={CloseUp}
                  />
                {:else}
                  <Icon
                    name="open-down"
                    size={14}
                    class="mx-1 flex-none"
                    iconColor="fill-current"
                    pathName={OpenDown}
                  />
                {/if}
              </IconButton>
            </span>

            <slot {item} list={tree_data} id={item.id}>
              {item.name}
            </slot>
          </summary>

          {#if item.children}
            <svelte:self tree_data={item.children} let:item let:list={tree_data} let:id>
              <slot {item} list={tree_data} id={item.id}>
                {item.name}
              </slot>
            </svelte:self>
          {/if}
        </details>
      {:else}
        <slot {item} list={tree_data} id={item.id}>
          {item.name}
        </slot>
      {/if}
    </li>
  {/each}
</ul>

<style>
  summary::-webkit-details-marker {
    display: none !important;
  }
  summary {
    list-style: none !important;
  }

  ul li ul li details summary {
    padding-left: 6px !important;
  }

  ul li ul li details summary ul li details summary ul li button {
    padding-left: 12px !important;
  }
  .selected {
    background-color: #1e6aff !important;
    color: white !important;
  }
  /* .selected *,
  .selected button {
    color: white !important;
  } */
</style>
