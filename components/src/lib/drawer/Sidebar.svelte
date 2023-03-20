<script lang="ts">
	import classNames from 'classnames';
	export let asideClass =
		'h-screen bg-white dark:bg-dark-2 border-r border-gray-primary dark:border-gray-brand-1 transition-width duration-200 ease-in';
	export let nameClass =
		'text-md font-bold text-body-light dark:text-body-dark text-center leading-[1.27] mx-4 mb-2';
	export let descClass =
		'font-medium text-default text-body dark:text-white leading-normal text-center mx-4 mt-2';
	export let divClass = 'border-b border-gray-primary py-4';
	export let style = '';
	let className = '';
	export { className as class };
	export let isClosed = true;
	export let name = 'This is a Project Name!';
	export let description =
		'This is a top-level description of a project that is created with a new event model.';
</script>

<aside class={classNames(className, asideClass)} {style}>
	<div class={divClass}>
		<h2 class={nameClass} contenteditable="true" bind:textContent={name}>{name}</h2>
		<p class={descClass} contenteditable="true" bind:textContent={description}>
			{description}
		</p>
	</div>
	<slot />
</aside>

<style>
	[contenteditable] {
		padding: 0.125em;
		/* border: 1px solid #eee; */
		/* border-radius: 4px; */
	}
</style>
