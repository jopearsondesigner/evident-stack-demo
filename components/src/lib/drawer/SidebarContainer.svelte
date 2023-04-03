<script>
	import classNames from 'classnames';
	import { slide } from 'svelte/transition';
	import { sineIn } from 'svelte/easing';

	export let brandClass =
		'inline bg-white dark:bg-dark-2 flex justify-center w-full cursor-default';
	export let btnClass =
		'pl-4 pr-1 h-8 transition duration-200 ease-in w-full bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] focus:text-body focus:bg-focus/[.20] dark:focus:bg-focus/[.20] cursor-default font-extrabold text-default text-body dark:text-white text-left whitespace-nowrap';

	/**
	 * Specify the accordion item title text.
	 * Alternatively, use the "title" slot
	 */
	export let title = 'Title';

	/**
	 * Set to `true` to expand the accordion item
	 */
	export let expanded = false;

	/**
	 * Specify the id
	 */
	export let id = 'item' + Math.random().toString(36);
	export let alt = '';
	export let height = 28;
	export let src = '';

	/**
	 * Obtain a reference to the `button` element
	 */
	export let ref = null;

	import { getContext, onMount } from 'svelte';

	const ctx = getContext('Accordion');

	let unsubscribe = undefined;

	onMount(() => {
		return () => {
			if (ctx) ctx.remove({ id });
			if (unsubscribe) unsubscribe();
		};
	});

	$: button_id = `button-${id}`;
	$: if (ctx) {
		ctx.add({ id, expanded });
		unsubscribe = ctx.items.subscribe((value) => {
			expanded = value[id];
		});
	}

	export let isLayerOpen = false;
	export let disabled = false;
	const handleLayer = () => {
		expanded = !expanded;
		disabled = !disabled;
	};

	export let isClosed = true;
</script>

<li data-accordion-item {...$$restProps}>
	<button
		bind:this={ref}
		type="button"
		aria-expanded={expanded}
		aria-controls={id}
		aria-disabled={disabled}
		id={button_id}
		on:click
		on:click={() => expanded()}
		disabled={expanded}
		class={expanded || isLayerOpen ? brandClass : btnClass}
		on:click={() => {
			if (ctx) {
				ctx.toggle({ id, expanded: !expanded });
				if (expanded && ref && ref.getBoundingClientRect().top < 0) {
					ref.scrollIntoView();
				}
			}
		}}
	>
		{#if expanded}
			<span class="py-6 px-1">
				<img {src} class={classNames()} {alt} {height} style="height:{height}px" />
			</span>
		{:else}
			<slot name="title">{title}</slot>
		{/if}
	</button>
	{#if expanded}
		<div
			role="region"
			{id}
			aria-labelledby={button_id}
			hidden={!expanded}
			class={classNames('h-[54vh]')}
			transition:slide={{ delay: 0, duration: 200, easing: sineIn }}
		>
			<slot />
		</div>
	{/if}
</li>
