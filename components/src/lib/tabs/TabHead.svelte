<script lang="ts">
  export let divClass = '';
  export let ulClass = 'flex';
</script>

<div class={divClass}>
  <ul class={ulClass} role="tablist">
    <slot />
  </ul>
</div>
