<script lang="ts">
	import classNames from 'classnames';
	import { clickOutside } from '../utils/clickOutside';

	import { onMount, setContext, createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { key } from '$lib/context/contextMenu.js';

	export let divClass: string =
		'absolute grid overflow-hidden whitespace-nowrap z-40 transform w-auto rounded-lg shadow-xl border border-light dark:border-border-dark bg-white dark:bg-dark-2 py-3';

	export let x;
	export let y;

	// whenever x and y is changed, restrict box to be within bounds
	$: (() => {
		if (!menuEl) return;

		const rect = menuEl.getBoundingClientRect();
		x = Math.min(window.innerWidth - rect.width, x);
		if (y > window.innerHeight - rect.height) y -= rect.height;
	})(x, y);

	const dispatch = createEventDispatcher();

	setContext(key, {
		dispatchClick: () => dispatch('click')
	});

	let menuEl;
	function onPageClick(e) {
		if (e.target === menuEl || menuEl.contains(e.target)) return;
		dispatch('clickoutside');
	}
</script>

<svelte:body on:click={onPageClick} />

<ul
	class={classNames(divClass)}
	transition:fade={{ duration: 100 }}
	bind:this={menuEl}
	style="top: {y}px; left: {x}px;"
>
	<slot />
</ul>
