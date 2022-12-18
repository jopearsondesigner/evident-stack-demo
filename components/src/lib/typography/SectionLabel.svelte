<script lang='ts'>
    export let labelClass:string='text-gray-primary dark:text-gray-brand-2 opacity-100 dark:opacity-60 font-bold pt-4 ml-4 text-left';
    export {labelClass as class};
</script>

<p class={labelClass}>
    <slot/>
</p>
