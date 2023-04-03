<script lang="ts">
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";

  export let data: PageData;

  let create_model = data.create_model!;

  const handleCreateEventModel = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let name = formData.get("name")?.toString();
    if (name) {
      let state = await create_model(name);
      goto(`/design/models/${state.id()}`)
    }
  };
</script>

<form on:submit|preventDefault={handleCreateEventModel}>
  <div class="form-control w-full max-w-xs">
    <label class="label" for="name">
      <span class="label-text">Name</span>
    </label>
    <input type="text" name="name" placeholder="Type here" class="input input-bordered w-full max-w-xs" />
  </div>
  <button type="submit">Create</button>
</form>
