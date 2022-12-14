<script>
	import classNames from 'classnames';
	import './styles/button.css';
	import { createEventDispatcher } from 'svelte';

	export let label = '';
	export let backgroundColor = '';
	export let primary = false;
	export let secondary = false;
	export let success = false;
	export let warning = false;

	export let textColor = 'dark';

	export let size = 'medium';

	let style = backgroundColor ? `background-color: ${backgroundColor}` : '';

	let mode = primary ? 'button--primary' : 'button--secondary';

	const dispatch = createEventDispatcher();

	/**
	 * @param {any} event
	 */
	function handleClick(event) {
		dispatch('click', event);
	}

	let className = '';
	export { className as class };

	export let classes = classNames(
		'button',
		$$props.class,
		mode,
		`button--${textColor}`,
		`button--${size}`
	);
</script>

<button
	type="button"
	class={classes}
	class:button--primary={primary}
	class:button--warning={warning}
	class:button--success={success}
	{style}
	on:click={handleClick}
>
	<slot>
		{label || 'Button'}
	</slot>
</button>
