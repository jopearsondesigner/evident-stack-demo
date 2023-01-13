<script lang="ts">
	export let heroClass = 'h-auto bg-white dark:bg-dark-1 text-body-light dark:text-body-dark';
</script>

<section class={heroClass}>
	<slot />
</section>
