<script lang="ts">
  export let row: number;
  export let column: number;

  $: gridRow = row + 1;
	$: gridColumn = column + 1;
</script>

<div
	class="cursor z-10 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300"
	style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
/>
