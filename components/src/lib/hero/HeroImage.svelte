<script lang="ts">
  import classNames from 'classnames';
  export let heroImage = 'h-full relative w-auto overflow-hidden';
  export let imgSize = '';
  export const alt: string = '';
  export const width: any = '';
  export const height: any = '';
  export const src: string = '';
  export const imgClass: string = '';
</script>

<div class={classNames(imgSize, heroImage)}>
  <slot />
</div>
