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
	<Select class="mt-2" items={types} bind:value={selected} placeholder="Choose Interface Type" />
</Label>
