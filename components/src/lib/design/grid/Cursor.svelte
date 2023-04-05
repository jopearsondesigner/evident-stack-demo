<script lang="ts">
  import { tick } from "svelte";
  import type { EventPlacement, InterfacePlacement, TimelinePlacement } from "../Grid";

  export let row: number;
  export let column: number;
  export let editing: boolean;
  export let placement: InterfacePlacement | TimelinePlacement | EventPlacement | null = null;
  export let rename_placement: (placement: string, name: string) => any = (p, n) => console.log("rename_placement", p, n);
  export let place_component: (name: string, kind: 'interface'|'command'|'event'|'readModel') => any = (n, t) => console.log("place_component", n, t)

  $: gridRow = row + 1;
  $: gridColumn = column + 1;

  let input: HTMLInputElement;

  const focusInput = async () => {
    await tick();
    input.focus();
  }

  $: if (editing) {
    focusInput()
  }

  let element: HTMLDivElement;

  const scrollIntoView = async () => {
    await tick()
    element.scrollIntoView({behavior: "smooth", block: "nearest", inline: "center"})
  }

  $: if (element && gridRow > 0 && gridColumn > 0) {
    scrollIntoView()
  }

  const handleSubmit = (e: SubmitEvent) => {
    let data = new FormData(e.target as HTMLFormElement);
    let name = data.get("name")?.toString();
    if (name) {
      if (placement) {
        rename_placement(placement.id, name)
      } else {
        // TODO: type from lane, or disambiguation if for timeline
        const kind = 'command';
        place_component(name, kind)
      }
    }
  }
</script>

{#if editing}
  <div
    bind:this={element}
    class="cursor z-20 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300 bg-gray-canvas dark:bg-dark-1"
    style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};">
    <form class="w-full, h-full" on:submit|preventDefault={handleSubmit}>
      <input name="name" class="w-full" type="text" value={placement?.name || ''} bind:this={input} />
    </form>
  </div>
{:else}
  <div
    bind:this={element}
    class="cursor z-20 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300"
    style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};" />
  {/if}
