<script>
	import Select from '$lib/form/Select.svelte';
	import Label from '$lib/form/Label.svelte';

	/**
	 * @type {any}
	 */
	let selected;
	let types = [
		{ value: 'blank', name: 'Blank' },
		{ value: 'figma', name: 'Figma' },
		{ value: 'image', name: 'Image' },
		{ value: 'job', name: 'Job' }
	];
</script>

<Label
	>Interface Type
	<Select class="mt-1" items={types} bind:value={selected} placeholder="Choose Interface Type" />
</Label>

<style>
	select {
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
		/* Remove default arrow */
		background-image: url(...);
		/* Add custom arrow */
	}
</style>
