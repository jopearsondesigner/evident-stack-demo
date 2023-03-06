<script>
	export let tip;
</script>

<!-- TODO: support HTML (i.e. Markdown output) -->

{#if tip}
	<div class="tooltip before:text-xs before:text-gray-primary" data-tip={tip}><slot /></div>
{:else}
	<slot />
{/if}
