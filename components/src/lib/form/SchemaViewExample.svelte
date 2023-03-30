<script>
	import Select from '$lib/form/Select.svelte';
	let selected;
	let types = [
		{ value: 'blank', name: 'Blank' },
		{ value: 'figma', name: 'Figma' },
		{ value: 'image', name: 'Image' },
		{ value: 'job', name: 'Job' }
	];
	import TextArea from '$lib/form/Textarea.svelte';
	import Label from '$lib/form/Label.svelte';
</script>

<Label
	>Interface Type
	<Select class="mt-2" items={types} bind:value={selected} />
</Label>

<TextArea
	textareaClass="text-mono bg-[#303841] block w-full text-sm border-0 px-0"
	value="&quot;domain&quot; &#123; id: string, type: string, date: int, &#125;"
/>
