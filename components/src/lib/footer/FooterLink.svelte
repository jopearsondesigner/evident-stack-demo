<script lang='ts'>
    export let liClass:string = 'w-full py-2';
    export let aClass:string = 'text-sm text-brand-gray-1 hover:text-gray-brand-4 dark:text-gray-brand-4 dark:hover:text-gray-brand-1 transition duration-200 ease-in';
    export {aClass as class};
    export let href:string = '/';
    export let label:string = '';
</script>

<li class={liClass}>
    <a {href} class={aClass}>
        {label}
    </a>
</li>
