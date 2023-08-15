<script lang="ts">
  import classNames from 'classnames';

  export let asideClass =
    'h-full flex-column bg-white dark:bg-dark-2 border-r border-gray-primary dark:border-border-dark transition-width duration-200 ease-in';

  export let style = '';
  let className = '';
  export { className as class };
</script>

<aside class={classNames(className, asideClass)} {style}>
  <slot />
</aside>
