<script lang='ts'>
    export let flexClass:string='flex justify-center items-center px-4';
    export {flexClass as class};
</script>

<div class={flexClass}>
    <slot/>
</div>
