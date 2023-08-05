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

  export let isClosed: any | boolean | never[] = [];
  export let summaryClass: string | undefined = '';
  export let btnClass: string | undefined = '';
</script>

<ul>
  {#each tree_data as item (item.id)}
    <li>
      {#if item.children}
        <details open>
          <summary
            class={classNames(
              summaryClass,
              'flex items-center bg-transparent hover:bg-focus/[.20] transition duration-200 ease-in w-full cursor-pointer'
            )}
            on:keyup={summaryKeyup}
            tabindex="0"
            on:click={() => (isClosed = !isClosed)}
            on:keyup
          >
            {#if !isClosed}
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
    padding-left: 6px;
  }
</style>
