<script lang='ts'>
    import classNames from 'classnames'
    import Logo from '../assets/images/global/evidentDesignLogo.svg';
    export let src = '';
	export let alt:string = "Brand Logo";
    export let href:string = '/';
    export let height:number = 32;
    export let logoClass='ml-4';
    export let product:string = '';
</script>

<!-- FooterBrand -->
<a {href}>
    <span class="sr-only">{product}</span>
    <img src={Logo} {alt} height={height} class={classNames(logoClass)} style="height:{height}px"/>
</a>
