<script>
	import Modal from '$lib/Modal.svelte';
	import Button from '$lib/Button.svelte';
	import Icon from '$lib/Icon.svelte';
	import File from '$lib/icons/File.svelte';
	import Checkmark from '$lib/icons/Checkmark.svelte';
	import ProgressBar from '$lib/utils/ProgressBar.svelte';

	let value = '100';
	export let done = false;

	let popupModal = false;
</script>

<div class="w-full text-center mt-4">
	<Button
		color="default"
		size="sm"
		on:click={() => (popupModal = true)}
		class=""
		label="Pop-up Modal"
	/>
</div>
<Modal bind:open={popupModal} size="xs" autoclose slot="header" title="Importing JSON File">
	<div class="text-center w-full inline-flex justify-center items-center p px-6">
		<Icon name="file" iconColor="text-body-light text-body-dark"><File /></Icon>
		<ProgressBar />
	</div>
	<div class="my-3 text-center w-full inline-flex justify-center items-center p px-6">
		{#if done}
			<Icon name="checkmark" iconColor="text-green" size={12}><Checkmark /></Icon>
		{:else}
			<Icon name="checkmark" iconColor="text-gray-primary" size={12}><Checkmark /></Icon>
		{/if}
		<span class="text-default text-body dark:text-white">Filename-Example-0.0.1</span>
	</div>
	<div slot="footer" class="mx-3 flex items-end">
		<Button color="default" size="sm" on:click class="" label="Done" />
	</div>
</Modal>
