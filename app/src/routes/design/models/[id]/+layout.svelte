<script lang="ts">
  import { goto } from '$app/navigation';
  import type { PageData } from './$types';
  import Modal from '$components/Modal.svelte';
  import Button from '$components/Button.svelte';
  import Icon from '$components/Icon.svelte';
  import Warning from '$components/icons/Warning.svelte';
  import Checkmark from '$components/icons/Checkmark.svelte';
  import Grid from '$components/design/Grid.svelte';

  export let data: PageData;
  const grid = data.grid;
  const import_json = data.import_json!;

  $: model_id = $grid?.id();

  let popupModal = false;
  let done = false;
  export let open = false;

  const hide = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    open = false;
  };

  const handleDeleteModel = () => {
    data.delete_model ? model_id : null;
    goto('/');
  };
  const handleImportJson = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let json = formData.get('json') as File;
    let buffer = await json.arrayBuffer();
    let bytes = new Uint8Array(buffer);
    let offset = formData.get('offset') as string;
    await import_json(model_id!, bytes, parseInt(offset) || 0);
  };
</script>

<h2 class="mt-16">{model_id}</h2>

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

<button
  class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
  on:click={() => (popupModal = true)}>Delete This Model</button
>

<Modal bind:open={popupModal} size="xs" autoclose title="Delete Event Model">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="warning" pathName={Warning} class="mr-1" />
    <span class="whitespace-nowrap text-sm text-body"
      >Are you sure you want to delete this Event Model?</span
    >
  </div>
  <div class="my-3 text-center w-full inline-flex justify-center items-center p px-6">
    {#if done}
      <Icon name="checkmark" iconColor="text-green" class="mr-1" size={12} pathName={Checkmark} />
    {:else}
      <Icon
        name="checkmark"
        iconColor="text-gray-primary"
        class="mr-1"
        size={12}
        pathName={Checkmark}
      />
    {/if}
    <span class="text-default text-body dark:text-white">{model_id}</span>
  </div>
  <div slot="footer" class="mx-3 flex items-end space-x-3">
    <Button color="default" size="sm" on:click={hide} class="" label="Cancel" />
    <Button
      gradient
      color="ghost"
      size="sm"
      on:click={handleDeleteModel}
      class=""
      label="Confirm"
    />
  </div>
</Modal>

<Modal bind:open={popupModal} size="xs" autoclose title="Delete Event Model">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="warning" pathName={Warning} class="mr-2" />
    <span class="whitespace-nowrap text-default text-body dark:text-white"
      >Are you sure you want to delete this Event Model?</span
    >
  </div>
  <div class="my-3 text-center w-full inline-flex justify-center items-center p px-6">
    {#if done}
      <Icon name="checkmark" iconColor="text-green" class="mr-1" size={12} pathName={Checkmark} />
    {:else}
      <Icon
        name="checkmark"
        iconColor="text-gray-primary"
        class="mr-1"
        size={12}
        pathName={Checkmark}
      />
    {/if}
    <span class="text-default text-body dark:text-body-dark">{model_id}</span>
  </div>
  <div slot="footer" class="mx-3 flex items-end space-x-3">
    <Button color="default" size="sm" on:click={hide} class="" label="Cancel" />
    <Button
      gradient
      color="ghost"
      size="sm"
      on:click={handleDeleteModel}
      class=""
      label="Confirm"
    />
  </div>
</Modal>

<Grid
  default_audience_placements={$grid?.default_audience}
  audiences={$grid?.audiences}
  timeline_placements={$grid?.timeline}
  streams={$grid?.streams}
  default_stream_placements={$grid?.default_stream}
/>

<slot />
