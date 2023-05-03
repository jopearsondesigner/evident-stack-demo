<script lang="ts">
  export let heroGridClass = 'grid lg:grid-cols-2 grid-col-1 h-full';
</script>

<div class={heroGridClass}>
  <slot />
</div>
