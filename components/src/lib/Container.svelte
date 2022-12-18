<script lang='ts'>
    export let containerClass: string | undefined='container';
    export {containerClass as class};
</script>

<section class={containerClass}>
    <slot/>
</section>
