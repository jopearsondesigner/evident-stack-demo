<script>
  export let data;

  let dispatch = data.dispatch;

  const handleCreateEventModel = async (e) => {
    const formData = new FormData(e.target);
    const data = {
      "Create": formData.get("name")
    };
    await dispatch(data);
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
