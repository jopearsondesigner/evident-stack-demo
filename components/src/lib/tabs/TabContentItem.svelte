<script lang="ts">
	import classNames from 'classnames';
	export let activeTabValue: number;
	export let id: number;
	export let contentDivClass = 'p-4 min-h-sreen h-full';
	import { fly } from 'svelte/transition';
	import { sineIn } from 'svelte/easing';
	let transitionParams = {
		x: 32,
		duration: 200,
		easing: sineIn,
		opacity: 100
	};
	let fadeOut = {
		duration: 0,
		opacity: 0
	};
</script>

{#if activeTabValue === id}
	<div
		class={classNames('lg:absolute', contentDivClass)}
		id="{id}-tabitem"
		role="tabpanel"
		aria-labelledby="{id}-tab"
		in:fly={transitionParams}
		out:fly={fadeOut}
	>
		<slot />
	</div>
{/if}
