<script lang="ts">
	export let formClass: string = 'p-4 w-full';
	export let action: string = '';
	export let method: string = 'post';
</script>

<form {action} {method} class={formClass}>
	<slot />
</form>
