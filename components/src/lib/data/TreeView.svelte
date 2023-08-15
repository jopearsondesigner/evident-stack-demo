<script context="module" lang="ts">
  export interface TreeItem {
    name: string;
    type: string;
    [id: number]: any;
    children?: TreeItem[];

    // To allow custom keys
    [key: string]: any;
  }

  export type TreeData = TreeItem[];
</script>

<script lang="ts">
  import classNames from 'classnames';
  import Icon from '../Icon.svelte';
  import IconButton from '../IconButton.svelte';
  import CloseUp from '../icons/CloseUp.svelte';
  import OpenDown from '../icons/OpenDown.svelte';

  export let tree_data: TreeData = [];

  function summaryKeyup(event: KeyboardEvent) {
    // @ts-ignore
    if (event.key == ' ' && document.activeElement.tagName != 'SUMMARY') {
      event.preventDefault();
    }
  }

  export let isClosed: any | boolean | never[] = [];
  let summaryClass: string | undefined = '';
  let btnClass: string | undefined = '';
  let iconBtnClass =
    'block text-current rounded-full border-none p-0 inline-flex items-center justify-center bg-gray-brand-1/0 dark:bg-white/0 hover:bg-gray-brand-1/[.09] dark:hover:bg-white/[.09] cursor-pointer';
  export let margin = 'm-px';
  export let ulClass = '';
  export let spanClass = '';
  export let isActive: string | number;
</script>

<ul class={ulClass}>
  {#each tree_data as item (item.id)}
    <li>
      {#if item.children}
        <details open>
          <summary
            class={classNames(
              summaryClass,
              'focus:pointer-events-none hover:pointer-events-auto text-body dark:text-white flex justify-start items-center w-full bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] transition duration-200 ease-in'
            )}
            class:selected={isActive === item.id}
            on:click={() => (isActive = item.id)}
            id={item.id}
            on:keyup={summaryKeyup}
            tabindex="0"
          >
            <span
              class={classNames('pointer-events-auto', spanClass)}
              on:click={() => (isClosed[item.id] = !isClosed[item.id])}
              on:keyup
            >
              {#if !isClosed[item.id]}
                <IconButton bind:iconBtnClass margin="mx-1" size={20}>
                  <Icon
                    name="close-up"
                    size={14}
                    class="mx-1 flex-none"
                    iconColor="fill-current"
                    pathName={CloseUp}
                  />
                </IconButton>
              {:else}
                <IconButton bind:iconBtnClass margin="mx-1" size={20}>
                  <Icon
                    name="open-down"
                    size={14}
                    class="mx-1 flex-none"
                    iconColor="fill-current"
                    pathName={OpenDown}
                  />
                </IconButton>
              {/if}
            </span>

            <button
              class={classNames(
                'grow text-default whitespace-nowrap text-body dark:text-white text-ellipsis leading-normal text-body dark:text-body-dark text-ellipsis flex items-center bg-transparent group-hover:bg-focus/[.20] transparent duration-200 ease-in'
              )}
            >
              <slot {item} list={tree_data} id={item.id}>
                {item.name}
              </slot>
            </button>
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
    color: white;
  }
  .selected *,
  .selected button {
    color: white !important;
  }
</style>
