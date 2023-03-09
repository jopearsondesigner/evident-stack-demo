<script lang="ts">
	import classNames from 'classnames';
	import Icon from './Icon.svelte';
	export let name = '';
	export let iconBtnClass =
		'w-9 h-9 rounded-full border-none p-0 inline-flex items-center justify-center bg-gray-brand-1/0 dark:bg-white/0 hover:bg-gray-brand-1/[.09] dark:hover:bg-white/[.09] transition duration-200 ease-in cursor-default';
	export let size = 20;
	export let pathName = '';
</script>

<button type="button" class={classNames(iconBtnClass)} on:click>
	<slot><Icon {name} {size} {pathName} /></slot>
</button>
