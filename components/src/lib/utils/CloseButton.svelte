<script lang="ts">
	import Icon from '../Icon.svelte';
	import Close from '../icons/Close.svelte';
	import { createEventDispatcher } from 'svelte';
	export let size: number | undefined = 16;
	const dispatch = createEventDispatcher();
	function handleClick(event: any) {
		dispatch('click', event);
	}
	export let btnClass = '';
	export let name = '';
</script>

<button type="button" on:click={handleClick} class={btnClass}>
	<span class="sr-only">Close</span>
	<Icon
		name="close"
		{size}
		viewBox="0 0 20 20"
		class="stroke-2"
		iconColor="text-body-light dark:text-gray-primary"
		pathName={Close}
	/>
</button>
