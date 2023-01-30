<script lang="ts">
	import classNames from 'classnames';
	export let pClass: string = 'text-body-light dark:text-body-dark leading-normal';
	export let textSize: string = 'text-base';
	export let classes: string = '';
</script>

<p {...$$restProps} class={classNames(textSize, pClass, classes, $$props.class)}>
	<slot />
</p>
