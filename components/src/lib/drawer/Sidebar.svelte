<script lang="ts">
  import classNames from 'classnames';

  export let asideClass =
    'flex flex-col h-full relative bg-white dark:bg-dark-2 border-r border-gray-primary dark:border-border-dark';

  export let navbarHeight: number = 64;

  export let style = '';
  let className = '';
  export { className as class };
</script>

<aside class={classNames(className, asideClass)} style="margin-top: {navbarHeight}px;">
  <slot />
</aside>
