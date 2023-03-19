<script lang="ts">
	import classNames from 'classnames';
	export let asideClass =
		'h-screen bg-white dark:bg-dark-2 border-r border-gray-primary dark:border-gray-brand-1 transition-width duration-200 ease-in';
	export let style = '';
	let className = '';
	export { className as class };
	export let isClosed = true;
</script>

<aside class={classNames('w-239px', className, asideClass)} {style}>
	<slot />
</aside>
