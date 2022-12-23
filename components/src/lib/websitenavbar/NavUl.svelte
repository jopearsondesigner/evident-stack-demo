<script lang='ts'>
    export let navUlClass='flex space-x-8 items-center';
    export { navUlClass as class };
    export let mobileNavUl='';
    export let hidden:boolean = false;
</script>

{#if !hidden}
<ul class={navUlClass}>
    <slot/>
</ul>
{:else}
<ul class={mobileNavUl}>
    <slot/>
</ul>
{/if}
