<script lang="ts">
	export let gradientDividerClass =
		'bg-gradient-to-t from-gradient to-transparency opacity-20 h-[64px]';
	export { gradientDividerClass as class };
</script>

<div class={gradientDividerClass}>
	<slot />
</div>
