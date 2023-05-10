<script lang="ts">
  import classNames from 'classnames';
  export let gridClass: string = 'grid grid-cols-1';
  export let customCols: string = '';
  export { gridClass as class };
  export let style: string = '';
</script>

<div class={classNames(gridClass, customCols)} {style}>
  <slot />
</div>
