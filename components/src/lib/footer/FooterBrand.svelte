<script lang='ts'>
    export let src = '';
	export let alt:string = "Brand Logo";
    export let href:string = '/';
    export let height:number = 24;
    export let logoClass='lg:h-[24px] h-[25px] ml-4';
    export let product:string = '';
</script>

<!-- FooterBrand -->
<a {href}>
    <span class="sr-only">{product}</span>
    <img class={logoClass} {src} {alt} {height}>
</a>
