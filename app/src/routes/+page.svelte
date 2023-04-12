<script lang="ts">
  import { handleSignOut } from '$lib/user';
  import type { PageData } from './$types';
  import Container from '$components/Container.svelte';
  import Row from '$components/Row.svelte';
  import Column from '$components/Column.svelte';
  import IndexNavButton from '$components/home/IndexNavButton.svelte';
  import Icon from '$components/Icon.svelte';
  import IconButton from '$components/IconButton.svelte';
  import Add from '$components/icons/Add.svelte';
  import Delete from '$components/icons/Delete.svelte';
  import Button from '$components/Button.svelte';
  let BackgroundImage = "bg-[url('$components/assets/images/product/design/modelThumb.svg')]";

  export let data: PageData;

  let project = [{ done: false }];

  function add() {
    project = project.concat({ done: false });
  }

  function clear() {
    project = project.filter((t) => !t.done);
  }

  $: remaining = project.filter((t) => !t.done).length;
</script>

{#if data.session.user}
  <section class="mt-20">
    <h1>Welcome</h1>
    <p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

    <button on:click|preventDefault={handleSignOut}>Sign Out</button>

    <h4>session:</h4>
    <pre>
    {JSON.stringify(data.session, null, 2)}
  </pre>

    <Button gradient color="brandStackPrimary" size="sm" href="/new" label="Create New Model" />
  </section>

  <p>{remaining} remaining</p>

  <button on:click={clear}>Clear completed</button>

  <Container class="container-fluid p-3 m-4">
    <Row class="grid grid-cols-6 gap-3">
      <Column>
        <IndexNavButton height={156} on:click={add}>
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
        <Column>
          <div class:done={project.done} class="w-full flex justify-center items-center">
            <input type="checkbox" bind:checked={project.done} />
            <IndexNavButton height={156} href="#" backgroundImg={BackgroundImage}>
              <div class="absolute self-start -mr-2 -mt-2">
                <IconButton
                  ><Icon
                    name="delete"
                    size={20}
                    iconColor="fill-current"
                    class="justify-self-end"
                    pathName={Delete}
                  /></IconButton
                >
              </div>
              <div class="inline-flex justify-center w-full">Untitled</div>
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
