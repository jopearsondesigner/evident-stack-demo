<script lang='ts'>
    export let ulClass='flex space-x-8 items-center';
    export { ulClass as class };
    export let mobileNavUl='';
    export let hidden:boolean = false;
</script>

{#if !hidden}
<ul class={ulClass}>
    <slot/>
</ul>
{:else}
<ul class={mobileNavUl}>
    <slot/>
</ul>
{/if}
