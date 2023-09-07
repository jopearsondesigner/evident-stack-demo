<script lang="ts">
  export let divClass = '';
  export let ulClass = 'no-scrollbar flex overflow-auto w-auto';
</script>

<div class={divClass}>
  <ul class={ulClass} role="tablist">
    <slot />
  </ul>
</div>

<style>
  /* Hide scrollbar*/
  .no-scrollbar::-webkit-scrollbar {
    display: none;
  }
</style>
