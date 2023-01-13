<script lang="ts">
	import classNames from 'classnames';
	export let divClass: string = '';
	export let style: string = '';
</script>

<div class={classNames(divClass, $$props.class)} {style}>
	<slot />
</div>
