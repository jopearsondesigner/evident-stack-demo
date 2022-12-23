<script lang='ts'>
    import Logo from '../assets/images/global/onoteLogo.svg';
	export let alt="Brand Logo";
    export let href: string;
    export let navBrandClass='flex min-w-[103.26px] no-underline';
    export { navBrandClass as class };
    export let logoClass='lg:h-[28px] h-[25px] ml-4';
    export let product='oNote';
</script>

<a {href}>
    <span class="sr-only">{product}</span>
    <img class={logoClass} src={Logo} {alt} height="32">
</a>

