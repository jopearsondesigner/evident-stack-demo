<script lang="ts">
	export let href = '/';
	export let alt: string;
	export let src: string;
	export let navBrandClass = 'mx-3 flex justify-center cursor-default h-[24px]';
</script>

<a {href} class={navBrandClass}>
	<img {src} height="22" class={navBrandClass} {alt} />
</a>
