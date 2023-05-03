<script lang="ts">
  import { goto } from '$app/navigation';
  import type { PageData } from './$types';
  import Modal from '$components/Modal.svelte';
  import Button from '$components/Button.svelte';
  import ProgressBar from '$components/utils/ProgressBar.svelte';
  import Icon from '$components/Icon.svelte';
  import Warning from '$components/icons/Warning.svelte';
  import FileIcon from '$components/icons/FileIcon.svelte';
  import Checkmark from '$components/icons/Checkmark.svelte';
  import Grid from '$components/design/Grid.svelte';

  export let data: PageData;
  const { grid, decider } = data;

  $: Modal_id = $grid?.id();

  let input: HTMLInputElement;
  let deleteModal = false;
  let importModal = false;
  let done = true;
  export let open = false;
  const hide = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    open = false;
  };

  const handleDeleteModel = async () => {
    await decider?.delete_model();
    goto('/');
  };

  const handleImportJson = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let json = formData.get('json') as File;
    let buffer = await json.arrayBuffer();
    let bytes = new Uint8Array(buffer);
    let offset = formData.get('offset') as string;
    await decider?.import_json(bytes, parseInt(offset) || 0);
  };

  const handleImportModel = () => (importModal = !importModal);
</script>

<form id="importModel" on:submit|preventDefault={handleImportJson}>
  <div class="w-full max-w-xs mt-16">
    <label class="label" for="json">
      <span class="label-text">Event Modal JSON File</span>
    </label>
    <input
      type="file"
      name="json"
      accept="application/json"
      on:change={handleImportModel}
      bind:this={input}
    />
    <label class="label" for="offset">
      <span class="label-text">Offset</span>
    </label>
    <input type="number" name="offset" class="input input-bordered w-full max-w-xs" />
  </div>
  <input type="submit" id="submit-form" class="hidden" on:click={handleImportModel} />
</form>

<button
  class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
  on:click={() => (deleteModal = true)}>Delete This Modal</button
>

<Modal bind:open={deleteModal} size="xs" autoclose title="Delete Event Modal">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="warning" pathName={Warning} class="mr-1" />
    <span class="whitespace-nowrap text-sm text-body"
      >Are you sure you want to delete this Event Modal?</span
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
    <span class="text-default text-body dark:text-white">{Modal_id}</span>
  </div>
  <div slot="footer" class="mx-3 flex items-end space-x-3">
    <Button color="default" size="sm" on:click={hide} class="" label="Cancel" />
    <Button gradient color="ghost" size="sm" label="confirm" on:click={handleDeleteModel} />
  </div>
</Modal>

<Modal bind:open={importModal} size="xs" autoclose title="Import JSON File">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="file" pathName={FileIcon} class="mr-1" />
    <ProgressBar done />
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
    <span class="text-default text-body dark:text-white">{Modal_id}</span>
  </div>
  <div slot="footer" class="mx-3 flex items-end space-x-3">
    <button
      class="text-sm text-focus hover:text-[#054FDE] transition duration-200 ease-in underline"
      on:click={hide}
      on:click={() => (input.value = '')}>Cancel</button
    >
    <Button input boundTo="submit-form" tabindex={0} color="default" size="sm" label="Done" />
  </div>
</Modal>

<Grid
  {decider}
  default_audience_placements={$grid?.default_audience}
  audiences={$grid?.audiences}
  timeline_placements={$grid?.timeline}
  streams={$grid?.streams}
  default_stream_placements={$grid?.default_stream}
  flows={$grid?.flows}
  column_count={$grid?.column_count || 0}
/>

<slot />
