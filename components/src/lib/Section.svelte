<script lang='ts'>
    import classNames from 'classnames';
    export let sectionClass: string = 'lg:mt-0 mt-8';
    export let divClass: string = '';
    let classes: string = '';
    export {classes as class};
    export let name:
        | 'benefit'
        | 'blog'
        | 'info'
        | 'resource';
    const sectionClasses = {
        benefit: {
            div: ''
        },
        blog: {
            div: ''
        },
        info: {
            div: ''
        },
        resource: {
            div: ''
        },
    }
</script>

<section class={classNames(sectionClass, $$props.classSection)}>
    <div class={classNames(sectionClasses[name]['div'], $$props.divClass)}>
        <slot/>
    </div>
</section>

