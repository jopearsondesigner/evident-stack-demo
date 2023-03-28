<script lang="ts">
  import { goto } from "$app/navigation";

  export let data;

  let dispatch = data.dispatch;

  const handleCreateEventModel = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    const data = {
      "Create": formData.get("name")
    };
    let state = await dispatch(data);
    goto(`./${state.EventModel.id}`)
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
