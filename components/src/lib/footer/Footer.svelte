<script lang="ts">
	export let footerClass: string = '';
	export { footerClass as class };
</script>

<footer class={footerClass}>
	<slot />
</footer>
