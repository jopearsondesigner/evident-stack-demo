<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let name: string;
  export let index: number;
  export let top: number = 0;
  export let left: number = 0;

  let dispatch = createEventDispatcher();

  const handleCommand = (_e: any) => {
    dispatch('define_and_place_command', {name, index})
  }
  const handleReadModel = (_e: any) => {
    dispatch('define_and_place_read_model', {name, index})
  }

  // TODO: keyboard handler for disambiguation
</script>

<ul class="menu absolute z-50 bg-base-100 w-56" style="top: {top}px; left: {left}px">
  <li><a href='/' on:click|preventDefault={handleCommand}>Command</a></li>
  <li><a href='/' on:click|preventDefault={handleReadModel}>Read Model</a></li>
</ul>
