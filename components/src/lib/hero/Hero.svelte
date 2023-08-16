<script lang="ts">
  export let heroClass = 'h-full bg-white dark:bg-dark-1 text-body-light dark:text-body-dark';
  export let style = '';
</script>

<section class={heroClass} {style}>
  <slot />
</section>
