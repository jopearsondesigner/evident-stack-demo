<script>
	import classNames from 'classnames';
	export let divClass = '';
	let className = '';
	export { className as class };
</script>

<div class={classNames(className, divClass)}>
	<slot />
</div>
