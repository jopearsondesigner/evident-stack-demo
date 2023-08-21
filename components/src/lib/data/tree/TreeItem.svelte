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
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  export let tree_data: TreeData = [];

  export let btnClass: string | undefined =
    'text-default whitespace-nowrap text-ellipsis leading-normal w-full';

  export let elementClass = '';

  // const handleClick = () => {
  //   goto(`/projects/${$page.params.id}/data/schemas/`);
  // };

  export let href = `/projects/${$page.params.id}/data/`;
</script>

<!-- <button class={classNames(btnClass, className)} on:click={handleClick}>
  <slot />
</button> -->

<svelte:element this={href ? 'a' : 'button'} {href} class={classNames(btnClass, elementClass)}>
  <slot />
</svelte:element>
