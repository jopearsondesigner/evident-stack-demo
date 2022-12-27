<script lang='ts'>
    export let navClass='hidden lg:flex-1 lg:flex lg:items-center lg:justify-between';
    export let navWebClass='flex items-center justify-between';
    export let website:boolean = false;
</script>

{#if !website}
<nav class={navClass}>
    <slot/>
</nav>
{:else}
<nav class={navWebClass}>
    <slot/>
</nav>
{/if}




