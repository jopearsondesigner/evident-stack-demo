<script lang='ts'>
    export let formClass:string = '';
    export let action:string = '';
    export let method:string = 'post';
</script>

<form {action} {method} class={formClass}>
    <slot/>
</form>