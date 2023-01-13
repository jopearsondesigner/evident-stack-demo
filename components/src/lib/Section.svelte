<script lang="ts">
	import classNames from 'classnames';
	export let sectionClass: string = 'lg:mt-0 mt-8';
	export let divClass: string = '';
	let classes: string = '';
	export { classes as class };
	export let name: 'general' | 'benefit' | 'blog' | 'info' | 'resource' | 'feature' | 'terms';
	const sectionClasses = {
		general: {
			div: ''
		},
		benefit: {
			div: ''
		},
		blog: {
			div: ''
		},
		info: {
			div: ''
		},
		resource: {
			div: ''
		},
		feature: {
			div: ''
		},
		terms: {
			div: ''
		}
	};
</script>

<section class={classNames(sectionClass, $$props.sectionClass)}>
	<div class={classNames(sectionClasses[name]['div'], $$props.divClass)}>
		<slot />
	</div>
</section>
