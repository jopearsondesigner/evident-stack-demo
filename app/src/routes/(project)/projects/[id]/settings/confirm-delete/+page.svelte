<script lang="ts">
  import Button from "$components/Button.svelte";
  import Icon from "$components/Icon.svelte";
  import Modal from "$components/Modal.svelte";
  import Warning from '$components/icons/Warning.svelte';
  import type { PageData } from "./$types";
  import { goto } from "$app/navigation";

  export let data: PageData;
  const { grid, decider } = data;

  $: event_model_name = $grid?.name;

  const handleDeleteModel = async () => {
    await decider?.delete_model();
    goto('/')
  };
</script>

<Modal open={true} size="xs" autoclose title="Delete Event Model">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="warning" pathName={Warning} class="mr-1" />
    <span class="whitespace-nowrap text-sm text-body">
      Are you sure you want to delete this Event Model?
    </span>
  </div>
  <div class="my-3 text-center w-full inline-flex justify-center items-center p px-6">
    <span class="text-default text-body dark:text-white">{event_model_name}</span>
  </div>
  <div slot="footer" class="mx-3 flex items-end space-x-3">
    <Button color="default" size="sm" on:click={() => goto("..")} class="" label="Cancel" />
    <Button gradient color="ghost" size="sm" label="confirm" on:click={handleDeleteModel} />
  </div>
</Modal>
