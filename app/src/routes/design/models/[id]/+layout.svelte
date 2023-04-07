<script lang="ts">
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";
  import Grid from "$components/design/Grid.svelte";

  export let data: PageData;
  const {grid, decider} = data;

  const handleDeleteModel = async () => { await decider?.delete_model(); goto('/') };
  const handleImportJson = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let json = formData.get("json") as File;
    let buffer = await json.arrayBuffer()
    let bytes = new Uint8Array(buffer)
    let offset = formData.get("offset") as string;
    await decider?.import_json(bytes, parseInt(offset) || 0);
  };
  console.warn("FLOWS");
  console.warn($grid?.flows);
</script>

<form on:submit|preventDefault={handleImportJson}>
  <div class="form-control w-full max-w-xs">
    <label class="label" for="json">
      <span class="label-text">Event Model JSON File</span>
    </label>
    <input type="file" name="json" accept="application/json" />
    <label class="label" for="offset">
      <span class="label-text">Offset</span>
    </label>
    <input type="number" name="offset" class="input input-bordered w-full max-w-xs" />
  </div>
  <button type="submit">Import</button>
</form>

<button on:click={handleDeleteModel}>Delete This Model</button>

<Grid
  {decider}
  default_audience_placements={$grid?.default_audience}
  audiences={$grid?.audiences}
  timeline_placements={$grid?.timeline}
  streams={$grid?.streams}
  default_stream_placements={$grid?.default_stream}
  flows={$grid?.flows}
  />

<slot />
