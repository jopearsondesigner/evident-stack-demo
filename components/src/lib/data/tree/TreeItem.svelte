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
  import { page } from '$app/stores';

  export let tree_data: TreeData = [];

  export let btnClass: string | undefined =
    'text-default whitespace-nowrap text-ellipsis leading-normal w-full';

  export let elementClass = '';
  export let href = '';
</script>

<a class={classNames(btnClass, elementClass)} {href} class:selected={$page.url.pathname === href}>
  <slot />
</a>

<style>
  .selected {
    background-color: #1e6aff !important;
    color: white !important;
  }
  .selected *,
  .selected button {
    color: white !important;
  }
</style>
