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
  import Button from '$components/Button.svelte';
  import modelThumb from '$components/assets/images/product/design/modelThumb.svg';
  import { goto } from '$app/navigation';

  export let data: PageData;

  const { projects } = data;

  let open = false;
  const hide = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    open = false;
  };

  const handleDeleteModel = async () => {
    console.log('TODO: delete model');
    goto('/');
  };
</script>

{#if data.session?.user}
  <Container class="container-fluid p-3 m-4">
    <Row class="grid grid-cols-6 gap-6">
      <Column class="flex justify-center items-center">
        <IndexNavButton href="/projects/new" height={156} rounded="rounded-lg">
          <div class="inline-flex justify-center w-full">
            Add New Project <Icon
                              name="add"
                              size={20}
                              iconColor="fill-current"
                              class=""
                              pathName={Add}/>
          </div>
        </IndexNavButton>
      </Column>
      {#each projects as project (project.id)}
        <Column class="flex justify-center items-center">
          <div class="w-full flex justify-center items-center">
            <IndexNavButton {modelThumb} height={156} href="/projects/{project.id}/design">
              <div class="absolute self-start -mr-2 -mt-2">
                <IconButton size={32} on:click={() => (open = true)} >
                  <Icon
                    name="delete"
                    size={16}
                    iconColor="fill-current"
                    class="justify-self-end"
                    pathName={Delete} />
                </IconButton>
              </div>
              {project.name}
            </IndexNavButton>
          </div>
        </Column>
      {/each}
    </Row>
  </Container>

  <Modal bind:open size="xs" autoclose title="Delete Event Modal">
    <div class="text-center w-full inline-flex justify-center items-center p px-6">
      <Icon name="warning" pathName={Warning} class="mr-1" />
      <span class="whitespace-nowrap text-sm text-body">
        Are you sure you want to delete this Event Modal?
      </span>
    </div>
    <div slot="footer" class="mx-3 flex items-end space-x-3">
      <Button color="default" size="sm" on:click={hide} class="" label="Cancel" />
      <Button gradient color="ghost" size="sm" label="confirm" on:click={handleDeleteModel} />
    </div>
  </Modal>
{:else}
  <p>
    Whoops, you're not signed in! Please <a class="text-primary" href="/auth/sign-in">sign in</a> to
    continue.
  </p>
{/if}
