<script>
  import Container from '$lib/Container.svelte';
  import Row from '$lib/Row.svelte';
  import Column from '$lib/Column.svelte';
  import IndexNavButton from '$lib/home/IndexNavButton.svelte';
  import Icon from '$lib/Icon.svelte';
  import IconButton from '$lib/IconButton.svelte';
  import Add from '$lib/icons/Add.svelte';
  import Delete from '$lib/icons/Delete.svelte';

  import ThemeSwitch from '$lib/utils/ThemeSwitch.svelte';

  let project = [{ done: false }];

  function add() {
    project = project.concat({ done: false });
  }

  function clear() {
    project = project.filter((t) => !t.done);
  }

  $: remaining = project.filter((t) => !t.done).length;
</script>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-8"><ThemeSwitch /></span>

<h1>Todos</h1>

<p>{remaining} remaining</p>

<!-- <button on:click={add}> Add new </button> -->

<button on:click={clear}>Clear completed</button>

<Container class="container-fluid p-3 m-4">
  <Row class="grid grid-cols-6 gap-3">
    <Column>
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
      <Column>
        <div class:done={project.done} class="w-full flex justify-center items-center">
          <input type="checkbox" bind:checked={project.done} />
          <IndexNavButton height={156} href="#">
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

<style>
  .done {
    opacity: 0.4;
  }
</style>
