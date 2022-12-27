<script lang='ts'>
    import classNames from 'classnames';
    import Logo from '../assets/images/global/onoteLogo.svg';
	export let alt="Brand Logo";
    export let href:string ='';
    export let brandClass='flex no-underline';
    export {brandClass as class};
    export let logoClass='ml-4';
    export let product='oNote';
    export let height:number;
</script>

<a {href}>
    <span class="sr-only">{product}</span>
    <img src={Logo} {alt} height={height} class={brandClass} style="height:{height}px"/>
</a>

