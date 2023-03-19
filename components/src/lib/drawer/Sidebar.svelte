<script lang="ts">
	import classNames from 'classnames';
	export let asideClass =
		'h-screen bg-white dark:bg-dark-2 border-r border-gray-primary dark:border-gray-brand-1 transition-width duration-200 ease-in';
	export let nameClass =
		'text-md font-bold text-body-light dark:text-body-dark text-center pt-4 px-4 pb-2 leading-[1.27]';
	export let descClass =
		'font-medium text-default text-body dark:text-white leading-normal text-center pt-2 px-4 pb-4';
	export let style = '';
	let className = '';
	export { className as class };
	export let isClosed = true;
</script>

<aside class={classNames(className, asideClass)} {style}>
	<div class="border-b border-gray-primary">
		<h2 class={nameClass}>This is a Project Name!</h2>
		<p class={descClass}>
			This is a top-level description of a project that is created with a new event model.
		</p>
	</div>
	<slot />
</aside>
