<script lang="ts">
	export let mainClass: string = '';
</script>

<main class={mainClass}>
	<slot />
</main>
