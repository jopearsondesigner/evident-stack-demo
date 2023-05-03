<script>
  import Modal from '$lib/Modal.svelte';
  import Button from '$lib/Button.svelte';
  import Icon from '$lib/Icon.svelte';
  import File from '$lib/icons/File.svelte';
  import Checkmark from '$lib/icons/Checkmark.svelte';
  import ProgressBar from '$lib/utils/ProgressBar.svelte';

  import ThemeSwitch from '$lib/utils/ThemeSwitch.svelte';

  let value = '100';
  export let done = false;

  let popupModal = false;
</script>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16"><ThemeSwitch /></span>

<div class="w-full text-center pt-12">
  <Button
    color="default"
    size="sm"
    on:click={() => (popupModal = true)}
    class=""
    label="Pop-up Modal"
  />
</div>
<Modal bind:open={popupModal} size="xs" autoclose title="Importing JSON File">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="file" iconColor="text-body-light dark:text-body-dark" pathName={File} />
    <ProgressBar />
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
    <span class="text-default text-body dark:text-white">Filename-Example-0.0.1</span>
  </div>
  <div slot="footer" class="mx-3 flex items-end">
    <Button color="default" size="sm" on:click class="" label="Done" />
  </div>
</Modal>
