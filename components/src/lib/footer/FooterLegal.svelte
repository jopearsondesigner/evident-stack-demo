<script lang='ts'>
    export let href:string = '/';
    export let aClass:string = 'text-sm text-brand-gray-1 hover:text-gray-brand-4 dark:text-gray-brand-4 dark:hover:text-gray-brand-1 transition duration-200 ease-in';
    export let spanClass:string = 'py-4 text-sm text-body-light dark:text-body-dark';
</script>

<!--MUST KEEP THESE LINKS UPDATED ALWAYS-->
<div class="text-center my-4">
    <a {href} class={aClass}>
        Privacy
    </a>
    <span class={spanClass}>|</span>
    <a {href} class={aClass}>
        Terms of Service
    </a>
</div>
