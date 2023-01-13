<script lang="ts">
	import classNames from 'classnames';
	export let ctaClass: string =
		'w-full bg-black bg-opacity-90 text-gray-primary flex justify-center items-center mx-0 px-1 py-2 text-right whitespace-nowrap';
	export let divClass: string = '';
	export let pClass: string = 'inline-flex pb-0 pr-4 whitespace-nowrap';
	export let label: string;
</script>

<div class={classNames(ctaClass, divClass)}>
	<p class={pClass}>
		{label}
	</p>
	<slot />
</div>
