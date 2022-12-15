<script>
	import classNames from 'classnames';
	import './navbar.css';
	export let toolbarClass = 'px-3 h-9 border-x border-gray-secondary flex items-center';
</script>

<div class={classNames(toolbarClass)}>
	<slot/>
</div>
