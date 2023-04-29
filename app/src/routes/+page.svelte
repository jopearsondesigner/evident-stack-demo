<script lang="ts">
  import type { PageData } from './$types';
  import Container from '$components/Container.svelte';
  import Row from '$components/Row.svelte';
  import Column from '$components/Column.svelte';
  import Modal from '$components/Modal.svelte';
  import IndexNavButton from '$components/home/IndexNavButton.svelte';
  import Icon from '$components/Icon.svelte';
  import IconButton from '$components/IconButton.svelte';
  import Add from '$components/icons/Add.svelte';
  import Delete from '$components/icons/Delete.svelte';
  import Warning from '$components/icons/Warning.svelte';
  import Checkmark from '$components/icons/Checkmark.svelte';
  import Button from '$components/Button.svelte';
  import modelThumb from '$components/assets/images/product/design/modelThumb.svg';
  import { goto } from '$app/navigation';

  export let data: PageData;

  $: Modal_id = 'modal-editing-window'; // TODO: derive this from model ID?

  let deleteModal = false;
  let project = [{ done: false }];
  let done = true;
  export let open = false;
  const hide = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    open = false;
  };

  function add() {
    project = project.concat({ done: false });
  }

  const handleDeleteModel = async () => {
    console.log('TODO: delete model');
    goto('/');
  };

  const handleCreateEventModel = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let name = formData.get('name')?.toString();
    if (name) {
      console.log('TODO: create model');
    }
  };

  function clear() {
    project = project.filter((t) => !t.done);
  }

  $: remaining = project.filter((t) => !t.done).length;
</script>

{#if data.session?.user}
  <section class="mt-20">
    <Button gradient color="brandStackPrimary" size="sm" href="/new" label="Create New Model" />
  </section>

  <p>{remaining} remaining</p>

  <button on:click={clear}>Clear completed</button>

  <Container class="container-fluid p-3 m-4">
    <Row class="grid grid-cols-6 gap-6">
      <Column class="flex justify-center items-center">
        <IndexNavButton height={156} on:click={add} rounded="rounded-lg">
          <div class="inline-flex justify-center w-full">
            Add New Project <Icon
              name="add"
              size={20}
              iconColor="fill-current"
              class=""
              pathName={Add}
            />
          </div>
        </IndexNavButton>
      </Column>
      {#each project as project}
        <Column class="flex justify-center items-center">
          <div class:done={project.done} class="w-full flex justify-center items-center">
            <input type="checkbox" bind:checked={project.done} />
            <IndexNavButton {modelThumb} height={156} href="#">
              <div class="absolute self-start -mr-2 -mt-2">
                <IconButton size={32} on:click={() => (deleteModal = true)}
                  ><Icon
                    name="delete"
                    size={16}
                    iconColor="fill-current"
                    class="justify-self-end"
                    pathName={Delete}
                  /></IconButton
                >
              </div>
              <form
                class="inline-flex justify-center w-full"
                on:submit|preventDefault={handleCreateEventModel}
              >
                <input
                  type="text"
                  name="name"
                  placeholder=""
                  value="Untitled"
                  class="group-hover:text-white group-hover:focus-visible:text-body group-hover:dark:focus-visible:text-white p-px group-hover:placeholder-white text-center w-full max-w-xs m-1 focus-visible:text-body dark:focus-visible:text-white focus-visible:border-0 focus-visible:outline-0 focus-visible:ring-focus focus-visible:ring-2 bg-transparent focus-visible:bg-white dark:focus-visible:bg-dark-1"
                />
              </form>
            </IndexNavButton>
          </div>
        </Column>
      {/each}
    </Row>
  </Container>
{:else}
  <p>
    Whoops, you're not signed in! Please <a class="text-primary" href="/auth/sign-in">sign in</a> to
    continue.
  </p>
{/if}

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
