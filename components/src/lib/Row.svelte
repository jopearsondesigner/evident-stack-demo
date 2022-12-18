<script lang='ts'>
    export let gridClass: string='grid lg:grid-cols-2 grid-col-1';
    export {gridClass as class};
    export let style:string = '';
</script>

<div class={gridClass} {style}>
    <slot/>
</div>
