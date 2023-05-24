<script lang="ts">
  import { goto } from '$app/navigation';
  import type { PageData } from './$types';
  import Button from '$components/Button.svelte';

  export let data: PageData;

  let create_model = data.create_model!;

  const handleCreateEventModel = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let name = formData.get('name')?.toString();
    if (name) {
      let state = await create_model(name);
      goto(`/projects/${state.id()}`);
    }
  };
</script>

<form on:submit|preventDefault={handleCreateEventModel}>
  <div class="form-control w-full max-w-xs">
    <label class="label" for="name">
      <span class="label-text">Name</span>
    </label>
    <input
      type="text"
      name="name"
      placeholder="Type here"
      class="input input-bordered w-full max-w-xs text-sm text-body dark:text-body-dark m-1 focus:border focus:ring-focus focus:border-focus focus-visible:border-0 focus-visible:outline-0 focus-visible:ring-focus focus-visible:ring-2 bg-transparent" />
  </div>
  <Button
    type="submit"
    gradient
    color="brandDesignPrimary"
    className="mt-4 ml-4"
    size="sm"
    label="Create" />
</form>
