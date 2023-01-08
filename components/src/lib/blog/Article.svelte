<script lang="ts">
    import classNames from 'classnames';
    export let articleClass: string =
      'flex flex-col justify-start h-full';
    export let customMargin: string = '';
  </script>

  <article class={classNames(articleClass, customMargin, $$props.class)}>
    <slot/>
  </article>