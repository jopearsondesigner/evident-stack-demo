<script lang="ts">
  import Drawer from '$components/drawer/Drawer.svelte';
  import Icon from '$components/Icon.svelte';
  import JobGears from '$components/icons/JobGears.svelte';
  import Button from '$components/Button.svelte';
  import Select from '$components/form/Select.svelte';
  import Input from '$components/form/Input.svelte';
  import Label from '$components/form/Label.svelte';
  // import ImageUpload from '$components/ImageUpload.svelte';
  import FigmaEmbed from '$components/FigmaEmbed.svelte';

  import type { PageData } from './$types';
  import type { InterfaceType } from '$components/design/Grid';
  import markdown from '$components/utils/markdown';
  import Textarea from '$components/form/Textarea.svelte';

  export let data: PageData;

  const { decider, grid, handle_close, placement } = data;

  let placement_kind_display = '';
  $: if (placement?.kind) {
    let kind = placement?.kind;
    if (kind == 'interface') {
      placement_kind_display = 'Interface';
    } else if (kind == 'command') {
      placement_kind_display = 'Command';
    } else if (kind == 'event') {
      placement_kind_display = 'Event';
    } else if (kind == 'read_model') {
      placement_kind_display = 'Read Model';
    }
  }
  let proposed_name = placement?.name;
  let proposed_description = placement?.description;
  $: proposed_description_html = markdown(proposed_description);
  let proposed_interface_config_kind: InterfaceType | undefined = undefined;
  let proposed_interface_config_url: string | undefined = undefined;

  const handleInterfaceConfigReset = () => {
    if (placement?.interface_config) {
      proposed_interface_config_kind = placement.interface_config.kind;
      proposed_interface_config_url = placement.interface_config.url;
    }
  }

  handleInterfaceConfigReset(); // Initialize to placement config

  // TODO: display effective schema for command/event/read-model placements, w/ link to edit in /data

  let placement_config_types = [
    { value: 'blank', name: 'Blank' },
    { value: 'figma', name: 'Figma' },
    { value: 'image', name: 'Image' },
    { value: 'job', name: 'Job' }
  ];

  const handleNameChange = async (_e: Event) => {
    if (decider && placement?.id && proposed_name) {
      await decider.rename_placement(placement.id, proposed_name);
    }
  };

  const handleDescriptionChange = async (e: Event) => {
    console.log('Updating name', e);
  };

  const handleUpdateInterfaceConfig = async (_e: SubmitEvent) => {
    if (decider && placement?.component_id && proposed_interface_config_kind) {
      try {
        await decider.configure_interface(
          placement.component_id,
          proposed_interface_config_kind,
          proposed_interface_config_url
        );
      } catch (e) {
        console.error("Error configuring placement", e, {
          component_id: placement?.component_id,
          proposed_interface_config_kind,
          proposed_interface_config_url
        });
      }
    }
  }
</script>

<svelte:head>
  <title>Placement Details | {placement?.name} | Design | {$grid?.name ?? "Project"} | Evident Stack</title>
</svelte:head>

<Drawer placement="right" class="" drawerRight hidden={false} on:close={handle_close}>
  <div slot="extra">
    {#if placement?.kind == 'interface'}
      <div class="flex flex-col items-start z-40 fixed top-1/2 -translate-y-1/2 left-1/2 -translate-x-2/3 h-3/4 w-1/2 p-3 bg-white dark:bg-dark-2">
        <h3 class="mt-1 mb-2 text-xl font-bold text-body-light dark:text-body-dark">{proposed_name}</h3>
        {#if proposed_description_html}
          <div>{proposed_description_html}</div>
        {/if}
        {#if proposed_interface_config_kind == 'figma'}
          <div class="flex-1 self-stretch min-h-0">
            <FigmaEmbed url={proposed_interface_config_url} />
          </div>
        {:else if proposed_interface_config_kind == 'image'}
          <img class="flex-1 self-center min-h-0 m-4" src={proposed_interface_config_url} alt={proposed_interface_config_url} />
        {:else if proposed_interface_config_kind == 'job'}
          <Icon class="flex-1 self-center min-h-0 m-10 w-max transition duration-200 ease-in cursor-default" name="job-gears" iconColor="text-body-light dark:text-body-dark" pathName={JobGears} />
        {/if}
      </div>
    {:else}
      <div class="flex flex-col items-stretch z-40 fixed top-1/2 h-3/4 -translate-y-1/2 w-1/2 left-1/2 -translate-x-2/3 p-3 bg-gradient-to-b from-{placement?.kind}-dark via-{placement?.kind} to-{placement?.kind}-light">
        <h3 class="mt-1 mb-2 text-xl font-bold">{proposed_name}</h3>
        {#if proposed_description_html}
          <div>{proposed_description_html}</div>
        {/if}
      </div>
    {/if}
  </div>
  {#if placement}
    <aside
      class="w-[480px] h-full py-6 flex items-center px-6 bg-white dark:bg-dark-2 border-l border-gray-primary dark:border-gray-brand-3">
      <div class="w-full h-full pb-6">
        <h3 class="text-left text-default font-extrabold text-body-light dark:text-body-dark mb-1">
          {placement_kind_display} Details
        </h3>
        <div
          class="w-full h-full flex flex-col items-stretch mb-6 p-6 border rounded border-border-light dark:border-border-dark">
          <Label for="name">
            Name
          </Label>
          <!-- TODO: green check saved indicator -->
          <Input name="name" type="text" tabindex="0" autofocus={true} bind:value={proposed_name} on:change={handleNameChange} />
          <Label for="description">
            Description
          </Label>
          <!-- TODO: green check saved/updated indicator -->
          <Textarea name="description" type="text" bind:value={proposed_description} on:change={handleDescriptionChange} />
          {#if placement.kind == 'interface' && proposed_interface_config_kind}
            <h4 class="mt-5 text-left text-default font-bold text-body-light dark:text-body-dark mb-1">
              Interface Config
            </h4>
            <form class="p-3 w-full" on:submit|preventDefault={handleUpdateInterfaceConfig}>
              <Select class="mt-2" placeholder="Choose Interface Type"
                      items={placement_config_types}
                      bind:value={proposed_interface_config_kind} />
              {#if proposed_interface_config_kind == 'figma'}
                <Input class="my-6" size="sm" placeholder="Figma URL" bind:value={proposed_interface_config_url} />
              {/if}
              {#if proposed_interface_config_kind == 'image'}
                <Input class="my-6" size="sm" placeholder="Image URL" bind:value={proposed_interface_config_url} />
              {/if}
              <div class="mt-6 space-x-3 col-span-1 place-self-center justify-self-end">
                <button
                  class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
                  on:click|preventDefault={handleInterfaceConfigReset}>
                  reset
                </button>
                <Button color="default" size="sm" label="Save" />
              </div>
            </form>
          {/if}
        </div>
      </div>
    </aside>
  {/if}
</Drawer>
