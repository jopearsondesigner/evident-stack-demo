<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import Button from '$components/Button.svelte';
  import Icon from '$components/Icon.svelte';
  import FileIcon from '$components/icons/FileIcon.svelte';
  import Modal from '$components/Modal.svelte';
  import { parseIntOr } from '$lib/util';
  import type { PageData } from './$types';

  export let data: PageData;

  const { decider } = data;

  let offset: number;
  $: offset = parseIntOr($page.url.searchParams.get('column'), 0);

  const handleClose = async () => {
    await goto(`/projects/${$page.params.id}/design`, { noScroll: true });
  }

  const handleImportJson = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let json = formData.get('json') as File;
    let buffer = await json.arrayBuffer();
    let bytes = new Uint8Array(buffer);
    await decider?.import_json(bytes, offset);
  };
</script>

<Modal open={true} size="xs" autoclose title="Import JSON File" on:close={handleClose}>
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="file" pathName={FileIcon} class="mr-1" />
    <form id="importModel" on:submit|preventDefault={handleImportJson}>
      <div class="w-full max-w-xs mt-16">
        <label class="label" for="json">
          <span class="label-text">Event Model JSON File</span>
        </label>
        <input
          type="file"
          name="json"
          accept="application/json" />
        <label class="label" for="offset">
          <span class="label-text">Offset</span>
        </label>
        <input type="number" name="offset" class="input input-bordered w-full max-w-xs" />
      </div>
      <input type="submit" id="submit-form" class="hidden" />
    </form>
  </div>
  <div slot="footer" class="mx-3 flex items-end space-x-3">
    <button
      class="text-sm text-focus hover:text-[#054FDE] transition duration-200 ease-in underline"
      on:click|preventDefault={handleClose}>Cancel</button>
    <Button input boundTo="submit-form" tabindex={0} color="default" size="sm" label="Done" />
  </div>
</Modal>
