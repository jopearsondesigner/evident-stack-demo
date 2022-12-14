<script>
	import classNames from 'classnames';
	import './navbar.css';
	export let navHamburgerClass = 'cursor-default';
	let hidden = true;
	let toggle = () => {
		hidden = !hidden;
	};
</script>

<button class={classNames(navHamburgerClass)} on:click={() => toggle()}>
	<alot name="icon" />
</button>
