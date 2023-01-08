<script lang='ts'>
    export let src: string = '';
	export let alt="Brand Logo";
    export let href:string ='';
    export let brandClass='flex no-underline';
    export {brandClass as class};
    export let logoClass='ml-4';
    export let product='Evident Design';
    export let height:number;
</script>

<a {href} class={brandClass}>
    <span class="sr-only">{product}</span>
    <img {src} {alt} height={height} class={logoClass} style="height:{height}px"/>
</a>

