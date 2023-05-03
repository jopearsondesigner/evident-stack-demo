<script>
	import classnames from 'classnames';
	export let ulClass = 'border-b border-gray-secondary dark:border-gray-brand-1 h-full';
	let className = '';
	export { ulClass as class };
</script>

<ul class={ulClass}>
	<slot />
</ul>
