<script lang="ts">
	export let heroImage = 'h-full relative w-auto overflow-hidden xl:w-[897px] lg:w-[640px]';
	export const alt: string = '';
	export const width: any = '';
	export const height: any = '';
	export const src: string = '';
	export const imgClass: string = '';
</script>

<div class={heroImage}>
	<slot />
</div>
