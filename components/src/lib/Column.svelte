<script lang='ts'>
    import classNames from 'classnames';
    export let flexClass: string = 'flex justify-center items-center px-4';
    export let customSpan: string = '';
    export {flexClass as class};
</script>

<div class={classNames(flexClass, customSpan, $$props.class)}>
    <slot/>
</div>
