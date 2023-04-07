<script lang="ts">
<<<<<<< HEAD
  import Icon from '../Icon.svelte';
  import Close from '../icons/Close.svelte';
  import { createEventDispatcher } from 'svelte';
  export let size = 16;
  const dispatch = createEventDispatcher();
  function handleClick(event: any) {
    dispatch('click', event);
  }
  export let btnClass = '';
=======
	import Icon from '../Icon.svelte';
	import Close from '../icons/Close.svelte';
	import { createEventDispatcher } from 'svelte';
	export let size = 16;
	export let style = '';
	const dispatch = createEventDispatcher();
	function handleClick(event: any) {
		dispatch('click', event);
	}
	export let btnClass = '';
>>>>>>> 1ee7d0f61df6b24b779798654431badc51036aef
</script>

<button type="button" on:click={handleClick} class={btnClass}>
  <span class="sr-only">Close</span>
  <Icon
    name="close"
    {size}
    viewBox="0 0 20 20"
    class="stroke-2"
    iconColor="text-body-light dark:text-gray-primary"><Close /></Icon
  >
</button>
