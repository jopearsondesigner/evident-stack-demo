<script lang="ts">
	export let heroFlexboxClass = 'flex justify-center';
	export { heroFlexboxClass as class };
</script>

<div class={heroFlexboxClass}>
	<slot />
</div>
