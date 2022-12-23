<script lang='ts'>
    export let ulClass:string = 'w-full px-4';
    export let liClass:string = 'pb-2 w-full border-b border-border-light dark:border-border-dark text-sm font-bold text-body-light dark:text-body-dark';
    export {ulClass as class};
    export let label:string = '';
</script>


<!-- FooterLinkGroup -->
<ul class={ulClass}>
    <li class={liClass}>
        {label}
    </li>
    <slot/>
</ul>
