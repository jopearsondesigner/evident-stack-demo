<script lang='ts'>
    export let liClass:string = 'w-full py-2';
    export let aClass:string = 'text-sm text-body-light hover:text-body-dark dark:text-body-dark dark:hover:text-body-light transition duration-200 ease-in';
    export {aClass as class};
    export let href:string = '/';
    export let label:string = '';
</script>

<li class={liClass}>
    <a {href} class={aClass}>
        {label}
    </a>
</li>
