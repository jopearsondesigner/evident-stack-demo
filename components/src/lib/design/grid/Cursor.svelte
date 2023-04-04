<script lang="ts">
  import { tick } from "svelte";


  export let row: number;
  export let column: number;

  let element: HTMLDivElement;

  $: gridRow = row + 1;
	$: gridColumn = column + 1;

  const scrollIntoView = async () => {
    await tick()
    element.scrollIntoView({behavior: "smooth", block: "start", inline: "center"})
  }

  $: if (element && gridRow > 0 && gridColumn > 0) {
    scrollIntoView()
  }
</script>

<div
  bind:this={element}
	class="cursor z-10 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300"
	style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
/>
