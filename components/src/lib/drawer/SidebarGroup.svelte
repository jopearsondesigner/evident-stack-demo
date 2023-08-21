<script>
  import classnames from 'classnames';
  export let ulClass = 'border-b border-border-light dark:border-border-dark';
  let className = '';
  export { ulClass as class };
</script>

<ul class={ulClass}>
  <slot />
</ul>
