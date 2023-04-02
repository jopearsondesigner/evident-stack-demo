<script>
	import ContextMenu from '$lib/context/ContextMenu.svelte';
	import ContextMenuItem from '$lib/context/ContextMenuItem.svelte';
	import ContextMenuDivider from '$lib/context/ContextMenuDivider.svelte';
	import { tick } from 'svelte';

	let pos = { x: 0, y: 0 };
	let showMenu = false;

	async function onRightClick(e) {
		if (showMenu) {
			showMenu = false;
			await new Promise((res) => setTimeout(res, 100));
		}

		pos = { x: e.clientX, y: e.clientY };
		showMenu = true;
	}

	function closeMenu() {
		showMenu = false;
	}
</script>

<h3 class="text-center mt-8">Right-Click Somewhere</h3>

{#if showMenu}
	<ContextMenu {...pos} on:click={closeMenu} on:clickoutside={closeMenu}>
		<ContextMenuItem>Add Event</ContextMenuItem>
		<ContextMenuDivider />
		<ContextMenuItem>Insert Column Left</ContextMenuItem>
		<ContextMenuItem>Insert Column Right</ContextMenuItem>
		<ContextMenuItem>Insert Lane Above</ContextMenuItem>
		<ContextMenuDivider />
		<ContextMenuItem>Import Event Model JSON</ContextMenuItem>
	</ContextMenu>
{/if}

<svelte:body on:contextmenu|preventDefault={onRightClick} />
