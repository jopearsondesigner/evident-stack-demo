<script>
	import classNames from 'classnames';
	import './drawer.css';
	export let ulClass = 'border-b border-gray-secondary';
</script>

<ul class={classNames(ulClass)}>
	<slot />
</ul>
