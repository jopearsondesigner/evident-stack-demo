<script context="module" lang="ts">
  export interface TreeItem {
    name: string;
    type: string;
    children?: TreeItem[];

    // To allow custom keys
    [key: string]: any;
  }

  export type TreeData = TreeItem[];
</script>

<script lang="ts">
  import classNames from 'classnames';
  import Icon from '$lib/Icon.svelte';
  import CloseUp from '$lib/icons/CloseUp.svelte';
  import OpenDown from '$lib/icons/OpenDown.svelte';

  export let tree_data: TreeData = [];

  function summaryKeyup(event: KeyboardEvent) {
    // @ts-ignore
    if (event.key == ' ' && document.activeElement.tagName != 'SUMMARY') {
      event.preventDefault();
    }
  }

  export let selected;
  export let id: any;
  export let type: any;
  export let isClosed: any | boolean | never[] = [];
  export let isActive: TreeItem | boolean;
  export let summaryClass = '';
  export let btnClass = '';
</script>

<ul class="">
  {#each tree_data as item, i}
    <li class="">
      {#if item.children}
        <details open>
          <summary
            class={classNames(
              summaryClass,
              'flex items-center bg-transparent hover:bg-focus/[.20] transition duration-200 ease-in w-full cursor-pointer'
            )}
            on:keyup={summaryKeyup}
            tabindex="0"
            on:click
            on:keyup
          >
            {#if !isClosed[i]}
              <Icon
                name="close-up"
                size={14}
                class="mx-1"
                iconColor="text-body-light dark:text-body-dark"
                pathName={CloseUp}
              />
            {:else}
              <Icon
                name="open-down"
                size={14}
                class="mx-1"
                iconColor="text-body-light dark:text-body-dark"
                pathName={OpenDown}
              />
            {/if}
            <slot {item} list={tree_data} id={i}>
              {item.name}
            </slot>
          </summary>

          {#if item.children}
            <svelte:self tree_data={item.children} let:item let:list={tree_data} let:id={i}>
              <div class="pl-2">
                <slot {item} list={tree_data} id={i}>
                  {item.name}
                </slot>
              </div>
            </svelte:self>
          {/if}
        </details>
      {:else}
        <button
          id={item[i]}
          class={classNames(
            btnClass,
            'pl-[22px] text-body dark:text-body-dark bg-transparent hover:bg-focus/[.20] transition duration-200 ease-in w-full'
          )}
          class:selected={isActive === item}
          on:click={() => (isActive = item)}
        >
          <slot {item} list={tree_data} id={i}>
            {item.name}
          </slot>
        </button>
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
  .selected {
    background-color: #1e6aff !important;
  }
  .selected {
    color: white;
  }
</style>
