<script lang='ts'>
    import classNames from 'classnames';
    export let heroFlexboxClass='flex justify-center';
    export {heroFlexboxClass as class};
</script>

<div class={heroFlexboxClass}>
    <slot/>
</div>
