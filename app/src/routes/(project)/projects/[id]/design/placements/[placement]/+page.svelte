<script lang="ts">
  import Drawer from '$components/drawer/Drawer.svelte';
  import Icon from '$components/Icon.svelte';
  import JobGears from '$components/icons/JobGears.svelte';
  import EventIcon from '$components/icons/EventIcon.svelte';
  import CommandIcon from '$components/icons/CommandIcon.svelte';
  import ReadModelIcon from '$components/icons/ReadModelIcon.svelte';
  import Button from '$components/Button.svelte';
  import Form from '$components/form/Form.svelte';
  import Select from '$components/form/Select.svelte';
  import Input from '$components/form/Input.svelte';
  import Label from '$components/form/Label.svelte';
  import ImageUpload from '$components/ImageUpload.svelte';
  import FigmaEmbed from '$components/FigmaEmbed.svelte';

  import CodeMirror from 'svelte-codemirror-editor';
  import { javascript } from '@codemirror/lang-javascript';
  import { oneDark } from '@codemirror/theme-one-dark';

  import type { PageData } from './$types';
  import type { InterfaceType } from '$components/design/Grid';
  import markdown from '$components/utils/markdown';

  export let data: PageData;

  const { decider, handle_close, placement } = data;

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
  if (placement?.interface_config) {
    proposed_interface_config_kind = placement.interface_config.kind;
    proposed_interface_config_url = placement.interface_config.url;
  }

  // TODO: display effective schema for command/event/read-model placements, w/ link to edit in /data

  let placement_config_types = [
    { value: 'blank', name: 'Blank' },
    { value: 'figma', name: 'Figma' },
    { value: 'image', name: 'Image' },
    { value: 'job', name: 'Job' }
  ];

  let iconShadow = 'shadow-[0_2.5px_4px_-2px_rgba(0,0,0,0.82)]';

  const handleEditDescription = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let name = formData.get('name')?.toString();
    if (name) {
      console.log('TODO: create model');
    }
  };

  let editable = false;
  function handleKeydown() {}
  function handleDblClick() {
    editable = !editable;
  }
</script>

<Drawer placement="right" class="" drawerRight hidden={false} on:close={handle_close}>
  <div slot="extra">
    {#if placement?.kind == 'interface'}
      <div class="flex flex-col items-start z-40 top-1/2 -translate-y-1/2 left-1/2 -translate-x-2/3 h-3/4 w-1/2 fixed bg-white dark:bg-dark-2">
        <h3>{proposed_name}</h3>
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
      <div class="z-40 top-1/2 h-3/4 -translate-y-1/2 w-1/2 left-1/2 -translate-x-2/3 fixed bg-{placement?.kind}">
        <h3>{proposed_name}</h3>
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
            <div class="inline-flex">
              <h2
                class="ml-3 self-end text-xl font-bold text-body-light dark:text-body-dark"
                on:dblclick={handleDblClick}
                on:blur={handleDblClick}
                on:keydown={handleKeydown}
                contenteditable={editable}>
                {placement.name}
              </h2>
            </div>
            <p
              class="my-3 text-sm leading-normal text-body dark:text-white"
              on:dblclick={handleDblClick}
              on:blur={handleDblClick}
              on:keydown={handleKeydown}
              contenteditable={editable}>
              {placement.description}
            </p>
            {#if placement.kind == 'interface' && proposed_interface_config_kind}
              <Form formClass="p-3 w-full">
                <Select class="mt-2" placeholder="Choose Interface Type" items={placement_config_types} bind:value={proposed_interface_config_kind} />
                {#if proposed_interface_config_kind == 'figma'}
                  <Input class="my-6" size="sm" placeholder="Figma URL" />
                {/if}
                {#if proposed_interface_config_kind == 'image'}
                  <Input class="my-6" size="sm" placeholder="Image URL" />
                {/if}
              </Form>
            {/if}
            <div class="grid grid-cols-2 justify-items-stretch">
              <div class="mt-6 col-span-1 place-self-center justify-self-start">
                <!-- <Button
                     label="Choose Image"
                     gradient
                     color="ghost"
                     size="sm"
                     className="my-4 justify-self-start chan"
                     on:click={() => {
                     fileinput.click();
                     }}
                     class="">
                     <Icon
                       slot="icon"
                       name="download"
                       size={12}
                       iconColor="text-body-light dark:text-white"
                       class="inline-flex mb-1 rotate-180"
                       pathName={Download} />
                </Button> -->
              </div>
              <div class="mt-6 space-x-3 col-span-1 place-self-center justify-self-end">
                <button
                  class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
                  on:click|preventDefault={handle_close}>
                  cancel
                </button>
                <Button color="default" size="sm" label="Save" on:click class="" />
              </div>
            </div>
          </div>
        </div>
    </aside>
  {/if}
</Drawer>
