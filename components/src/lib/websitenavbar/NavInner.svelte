<script lang="ts">
	export let navDivClass =
		'xl:container mx-auto flex justify-between items-center lg:py-0 lg:justify-start lg:space-x-8';
</script>

<div class={navDivClass}>
	<slot />
</div>
