<script lang='ts'>
    import Icon from '../Icon.svelte';
    import Menu from '../icons/Menu.svelte';
    import {createEventDispatcher} from 'svelte';
    export let navHamburgerClass='mr-4 lg:hidden bg-none rounded-md p-2 inline-flex items-center justify-center text-body-1 dark:text-body-4 no-underline';
    export { navHamburgerClass as class };
    const dispatch = createEventDispatcher();
	function handleClick(event: any) {
		dispatch('click', event);
	}
</script>

<button
type="button"
hidden
class={navHamburgerClass}
on:click={handleClick}>
    <span class="sr-only">Open menu</span>
    <Icon name='menu' size={24} viewBox='0 0 24 24' iconColor='dark:text-gray-primary'><Menu/></Icon>
</button>