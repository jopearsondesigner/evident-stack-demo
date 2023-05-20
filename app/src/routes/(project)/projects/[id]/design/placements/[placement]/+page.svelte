<script lang="ts">
  import Drawer from "$components/drawer/Drawer.svelte";
  import Icon from "$components/Icon.svelte";
  import EventIcon from '$components/icons/EventIcon.svelte';
  import CommandIcon from '$components/icons/CommandIcon.svelte';
  import ReadModelIcon from '$components/icons/ReadModelIcon.svelte';
  import InterfaceIcon from '$components/icons/InterfaceIcon.svelte';
  import InterfaceBlank from '$components/assets/images/product/global/InterfaceBlank.svg';
  import InterfaceJob from '$components/assets/images/product/global/InterfaceJob.svg';
  import Button from "$components/Button.svelte";
  import Form from "$components/form/Form.svelte";
  import Select from "$components/form/Select.svelte";
  import Input from "$components/form/Input.svelte";
  import Label from '$components/form/Label.svelte';

  import CodeMirror from 'svelte-codemirror-editor';
  import { javascript } from '@codemirror/lang-javascript';
  import { oneDark } from '@codemirror/theme-one-dark';

  import type { PageData } from "./$types";
  import ImageUpload from "$components/ImageUpload.svelte";

  export let data: PageData;

  const { grid } = data;

  $: name = $grid?.name;
  $: description = $grid?.description;

  let blank: boolean = false;
  let figma: boolean = false;
  let image: boolean = true;
  let job: boolean = false;
  let selected: string = '';

  let types = [
    { value: 'blank', name: 'Blank' },
    { value: 'figma', name: 'Figma' },
    { value: 'image', name: 'Image' },
    { value: 'job', name: 'Job' }
  ];

  let iconShadow = 'shadow-[0_2.5px_4px_-2px_rgba(0,0,0,0.82)]';
  let event: boolean = true;
  let command: boolean = false;
  let readModel: boolean = false;
  let hiddenRight: boolean = true;
  let placementDetails: boolean = true;
  let interfaceDetails: boolean = false;

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

  let valueComponent = '';
  let valuePlacement = '';
</script>

<style>
  [contenteditable] {
    padding: 0.125em;
    border-radius: 4px;
    border: none;
  }
  [contenteditable]:focus {
    outline: 1px solid #1e6aff;
  }
</style>

<Drawer placement="right" class="" bind:hidden={hiddenRight} drawerRight>
  <aside
    class="w-[480px] h-full py-6 flex items-center px-6 bg-white dark:bg-dark-2 border-l border-gray-primary dark:border-gray-brand-3">
    {#if placementDetails}
      <span class="w-full">
        <h3 class="text-left text-default font-extrabold text-body-light dark:text-body-dark mb-1">
          Placement Details
        </h3>
        <div class="w-full p-6 border rounded border-border-light dark:border-border-dark">
          <div class="grid grid-cols-6">
            <div class="col-span-1">
              {#if event}
                <Icon
                  name="event-icon"
                  size={48}
                  class={iconShadow}
                  iconColor=""
                  pathName={EventIcon}
                  viewBox="0 0 48 48" />
                {:else if command}
                  <Icon
                    name="command-icon"
                    size={48}
                    class={iconShadow}
                    iconColor=""
                    pathName={CommandIcon}
                    viewBox="0 0 48 48"
                    />
                  {:else if readModel}
                    <Icon
                      name="read-model-icon"
                      size={48}
                      class={iconShadow}
                      iconColor=""
                      pathName={ReadModelIcon}
                      viewBox="0 0 48 48"
                      />
                    {/if}
                  </div>
            <h2
              class="col-span-5 self-end text-xl font-bold text-body-light dark:text-body-dark"
              contenteditable="true"
              bind:textContent={name}>
              {name}
            </h2>
          </div>
          <p
            class="my-3 text-sm leading-normal text-body dark:text-white"
            contenteditable="true"
            bind:textContent={description}>
            {description}
          </p>

          <div class="my-3">
            <Label class="mt-4"
                   ><span class="mb-1 text-body dark:text-white">Component Schema</span></Label
                                                                                          >
            <CodeMirror
              value={valueComponent}
              theme={oneDark}
              lang={javascript()}
              styles={{
              '&': {
              width: '100%',
              height: '8.875rem',
              backgroundColor: '#303841',
              fontFamily: 'input-mono',
              color: '#D8DEE9'
              }
              }}
              class="mt-1"
              />
          </div>
          <div class="my-3">
            <Label class="mt-4">
              <span class="mb-1 text-body dark:text-white">Placement Schema</span>
            </Label>
            <CodeMirror
              value={valuePlacement}
              theme={oneDark}
              lang={javascript()}
              styles={{
              '&': {
              width: '100%',
              height: '8.875rem',
              backgroundColor: '#303841',
              fontFamily: 'input-mono',
              color: '#D8DEE9'
              }
              }}
              class="mt-1" />
          </div>
          <div class="mt-6 mx-3 space-x-3 flex justify-end">
            <button
              class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
              on:click={() => (hiddenRight = true)}>
              cancel
            </button>
            <Button color="default" size="sm" label="Save" on:click class="" />
          </div>
        </div>
      </span>
    {:else if interfaceDetails}
      <div class="w-full h-full pb-6">
        <h3 class="text-left text-default font-extrabold text-body-light dark:text-body-dark mb-1">
          Interface Details
        </h3>
        <div
          class="w-full h-full flex flex-col items-stretch mb-6 p-6 border rounded border-border-light dark:border-border-dark">
          <div class="inline-flex">
            <Icon
              name="event-icon"
              size={48}
              class="drop-shadow-[0_2px_1px_rgba(0,0,0,0.44)]"
              iconColor=""
              pathName={InterfaceIcon}
              viewBox="0 0 48 48" />
            <h2
              class="ml-3 self-end text-xl font-bold text-body-light dark:text-body-dark"
              on:dblclick={handleDblClick}
              on:blur={handleDblClick}
              on:keydown={handleKeydown}
              contenteditable={editable}>
              Interface Name
            </h2>
          </div>
          <p
            class="my-3 text-sm leading-normal text-body dark:text-white"
            on:dblclick={handleDblClick}
            on:blur={handleDblClick}
            on:keydown={handleKeydown}
            contenteditable={editable}>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
            incididunt ut labore et dolore magna aliqua.
          </p>
          <Form formClass="p-3 w-full">
            <Select class="mt-2" placeholder="Choose Interface Type" items={types} bind:value={selected} />
            {#if figma}
              <Input class="my-6" size="sm" placeholder="Figma URL" />
            {/if}
            {#if image}
              <Input class="my-6" size="sm" placeholder="Image URL" />
            {/if}
          </Form>
          <div class="h-full">
            {#if figma}
              <iframe
                title="Figma example"
                style="border: 1px solid rgba(0, 0, 0, 0.1);"
                width="100%"
                height="100%"
                src="https://www.figma.com/embed?embed_host=share&url=https%3A%2F%2Fwww.figma.com%2Ffile%2FkJQbEXGjeuvLde5DGLjUuP%2FAutonomous-Two-sided-Marketplace%3Fnode-id%3D0%253A1%26t%3DCHFyL6I3kTDpsftc-1"
                allowfullscreen />
              {:else if blank}
                <div class="h-full flex justify-center items-center">
                  <img src={InterfaceBlank} height="142" width="auto" alt="Blank Interface" />
                </div>
              {:else if image}
                <div class="h-full flex flex-col justify-center items-center">
                  <ImageUpload />
                </div>
              {:else if job}
                <div class="h-full flex justify-center items-center">
                  <img src={InterfaceJob} height="142" width="auto" alt="Blank Interface" />
                </div>
              {/if}
            </div>
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
                   class=""
                   ><Icon
                   slot="icon"
                   name="download"
                   size={12}
                   iconColor="text-body-light dark:text-white"
                   class="inline-flex mb-1 rotate-180"
                   pathName={Download}
                   /></Button
                   > -->
            </div>
            <div class="mt-6 space-x-3 col-span-1 place-self-center justify-self-end">
              <button
                class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
                on:click={() => (hiddenRight = true)}>
                cancel
              </button>
              <Button color="default" size="sm" label="Save" on:click class="" />
            </div>
          </div>
        </div>
      </div>
    {/if}
  </aside>
</Drawer>
