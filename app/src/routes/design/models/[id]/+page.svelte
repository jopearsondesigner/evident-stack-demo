<script lang="ts">
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";

  export let data: PageData;
  const grid = data.grid;
  const import_json = data.import_json!;

  $: model_id = $grid?.id;

  const handleDeleteModel = () => { data.delete_model?(model_id) : null; goto('/') };
  const handleImportJson = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let json = formData.get("json") as File;
    let buffer = await json.arrayBuffer()
    let bytes = new Uint8Array(buffer)
    let offset = formData.get("offset") as string;
    let state = await import_json(model_id, bytes, parseInt(offset) || 0);
    goto(`/design/models/${state.EventModel.id}`)
  };
</script>

<h2>{model_id}</h2>

<pre>{JSON.stringify($grid)}</pre>

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
