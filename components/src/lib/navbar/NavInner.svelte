<script>
	import classNames from 'classnames';
	import './navbar.css';
	export let navDivCLass = 'flex justify-between items-center';
</script>

<div class={classNames(navDivCLass)}>
	<slot/>
</div>
