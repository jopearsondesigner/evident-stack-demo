<script lang="ts">
    import classNames from 'classnames';

    export let href: string = '#';
    export let color: string = 'text-focus hover:text-[#054FDE]';
    export let aClass: string = 'inline-flex items-center transition duration-300 ease-in';
    export {aClass as class};
  </script>

  <a
  on:click
  {...$$restProps} {href} class={classNames(aClass, color, $$props.class)}>
    <slot />
  </a>