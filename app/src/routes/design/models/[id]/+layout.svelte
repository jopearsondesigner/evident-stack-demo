<script lang="ts">
  import { goto } from '$app/navigation';
  import type { PageData } from './$types';
  import Modal from '$components/Modal.svelte';
  import Button from '$components/Button.svelte';
  import Icon from '$components/Icon.svelte';
  import IconButton from '$components/IconButton.svelte';
  import ProgressBar from '$components/utils/ProgressBar.svelte';
  import Warning from '$components/icons/Warning.svelte';
  import FileIcon from '$components/icons/FileIcon.svelte';
  import Checkmark from '$components/icons/Checkmark.svelte';
  import Grid from '$components/design/Grid.svelte';

  import Drawer from '$components/drawer/Drawer.svelte';
  import Form from '$components/form/Form.svelte';
  import Textarea from '$components/form/Textarea.svelte';
  import Label from '$components/form/Label.svelte';
  import Input from '$components/form/Input.svelte';
  import Select from '$components/form/Select.svelte';
  import ArrowDown from '$components/assets/images/global/ArrowDown.svg';
  let blank: boolean = false;
  let figma: boolean = false;
  let image: boolean = true;
  let job: boolean = false;
  let selected: string = '';
  let ImgUrl: string =
    'https://cdn.dribbble.com/users/4838879/screenshots/15120947/media/92b243f9e1089780124a08943ed44f92.jpg';

  export let value: string | number;
  let types = [
    { value: 'blank', name: 'Blank' },
    { value: 'figma', name: 'Figma' },
    { value: 'image', name: 'Image' },
    { value: 'job', name: 'Job' }
  ];

  import EventIcon from '$components/icons/EventIcon.svelte';
  import CommandIcon from '$components/icons/CommandIcon.svelte';
  import ReadModelIcon from '$components/icons/ReadModelIcon.svelte';
  import InterfaceIcon from '$components/icons/InterfaceIcon.svelte';
  import InterfaceBlank from '$components/assets/images/product/global/InterfaceBlank.svg';
  import InterfaceJob from '$components/assets/images/product/global/InterfaceJob.svg';
  import ImgPlaceholder from '$components/assets/images/product/global/ImagePlaceholder.svg';
  import Download from '$components/icons/Download.svelte';
  import ImageUpload from '$components/icons/ImageUpload.svelte';

  let iconShadow = 'shadow-[0_2.5px_4px_-2px_rgba(0,0,0,0.82)]';
  let code: string =
    '"domain" {\n' + '\tfoo: string, \n' + '\tbar: string, \n' + '\tbaz: int\n' + '}';
  let event: boolean = true;
  let command: boolean = false;
  let readModel: boolean = false;
  let hiddenRight: boolean = true;
  let placementDetails: boolean = false;
  let interfaceDetails: boolean = true;

  export let data: PageData;
  const { grid, decider } = data;

  $: Modal_id = $grid?.id();

  let input: HTMLInputElement;
  let deleteModal = false;
  let importModal = false;
  let done = true;
  export let open = false;
  const hide = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    open = false;
  };

  const handleDeleteModel = async () => {
    await decider?.delete_model();
    goto('/');
  };

  const handleImportJson = async (e: SubmitEvent) => {
    const formData = new FormData(e.target as HTMLFormElement);
    let json = formData.get('json') as File;
    let buffer = await json.arrayBuffer();
    let bytes = new Uint8Array(buffer);
    let offset = formData.get('offset') as string;
    await decider?.import_json(bytes, parseInt(offset) || 0);
  };

  const handleImportModel = () => (importModal = !importModal);

  let editable = false;
  function handleKeydown() {}
  function handleDblClick() {
    editable = !editable;
  }

  let imgUpload: string | null | undefined, fileinput: HTMLInputElement;

  const onFileSelected = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
    let image = e.target.files[0];
    let reader = new FileReader();
    reader.readAsDataURL(image);
    reader.onload = (e) => {
      imgUpload = e.target.result;
    };
  };
</script>

<form id="importModel" on:submit|preventDefault={handleImportJson}>
  <div class="w-full max-w-xs mt-16">
    <label class="label" for="json">
      <span class="label-text">Event Modal JSON File</span>
    </label>
    <input
      type="file"
      name="json"
      accept="application/json"
      on:change={handleImportModel}
      bind:this={input}
    />
    <label class="label" for="offset">
      <span class="label-text">Offset</span>
    </label>
    <input type="number" name="offset" class="input input-bordered w-full max-w-xs" />
  </div>
  <input type="submit" id="submit-form" class="hidden" on:click={handleImportModel} />
</form>

<button
  class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
  on:click={() => (deleteModal = true)}>Delete This Modal</button
>

<div class="text-center z-[31] absolute inset-x-0 pt-4 mt-8">
  <!--For testing-->
  <Button
    gradient
    color="primary"
    size="sm"
    on:click={() => (hiddenRight = false)}
    class=""
    label="Show Sidebar"
  />
  <br />
  <button on:click={() => (hiddenRight = true)} class="mt-4">Close</button>
</div>

<Drawer placement="right" className="" bind:hidden={hiddenRight} drawerRight>
  <aside
    class="w-[480px] h-full py-6 flex items-center px-6 bg-white dark:bg-dark-2 border-l border-gray-primary dark:border-gray-brand-3"
  >
    {#if placementDetails}
      <span class="w-full">
        <h3 class="text-left text-default font-extrabold text-body-light dark:text-body-dark mb-1">
          Placement Details
        </h3>
        <div class="w-full p-6 border rounded border-border-light dark:border-border-dark">
          <div class="inline-flex">
            {#if event}
              <Icon
                name="event-icon"
                size={48}
                class={iconShadow}
                iconColor=""
                pathName={EventIcon}
                viewBox="0 0 48 48"
              />
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
            <h2
              class="ml-3 self-end text-xl font-bold text-body-light dark:text-body-dark"
              on:dblclick={handleDblClick}
              on:blur={handleDblClick}
              on:keydown={handleKeydown}
              contenteditable={editable}
            >
              Component Name
            </h2>
          </div>
          <p
            class="my-3 text-sm leading-normal text-body dark:text-white"
            on:dblclick={handleDblClick}
            on:blur={handleDblClick}
            on:keydown={handleKeydown}
            contenteditable={editable}
          >
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
            incididunt ut labore et dolore magna aliqua.
          </p>
          <div class="py-3">
            <Label color="default"
              ><span class="text-body dark:text-white">Component Schema</span>
              <Textarea
                placeholder=""
                value={code}
                name="description"
                rows="6"
                class="mt-1 font-mono block w-full overflow-auto text-sm border border-border-light dark:border-border-dark px-10 py-2.5"
                style="background-color: rgba(48, 56, 65, 100%); color: #D8DEE9;"
                disabled
              />
            </Label>
          </div>
          <div class="py-3">
            <Label color="default"
              ><span class="text-body dark:text-white">Placement Schema</span>
              <Textarea
                placeholder=""
                value={code}
                name="description"
                rows="6"
                class="mt-1 font-mono block w-full overflow-auto text-sm border border-border-light dark:border-border-dark px-10 py-2.5"
                style="background-color: rgba(48, 56, 65, 100%); color: #D8DEE9;"
                disabled
              />
            </Label>
          </div>
          <div class="mt-6 mx-3 space-x-3 flex justify-end">
            <button
              class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
              on:click={() => (hiddenRight = true)}>cancel</button
            >
            <Button color="default" size="sm" label="Edit" on:click class="" />
          </div>
        </div>
      </span>
    {:else if interfaceDetails}
      <span class="w-full h-full pb-6">
        <h3 class="text-left text-default font-extrabold text-body-light dark:text-body-dark mb-1">
          Interface Details
        </h3>
        <div
          class="w-full h-full flex flex-col items-stretch mb-6 p-6 border rounded border-border-light dark:border-border-dark"
        >
          <div class="inline-flex">
            <Icon
              name="event-icon"
              size={48}
              class="drop-shadow-[0_2px_1px_rgba(0,0,0,0.44)]"
              iconColor=""
              pathName={InterfaceIcon}
              viewBox="0 0 48 48"
            />
            <h2
              class="ml-3 self-end text-xl font-bold text-body-light dark:text-body-dark"
              on:dblclick={handleDblClick}
              on:blur={handleDblClick}
              on:keydown={handleKeydown}
              contenteditable={editable}
            >
              Interface Name
            </h2>
          </div>
          <p
            class="my-3 text-sm leading-normal text-body dark:text-white"
            on:dblclick={handleDblClick}
            on:blur={handleDblClick}
            on:keydown={handleKeydown}
            contenteditable={editable}
          >
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
            incididunt ut labore et dolore magna aliqua.
          </p>
          <Form formClass="p-3 w-full">
            <Select class="mt-2" items={types} bind:value={selected} />
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
                allowfullscreen
              />
            {:else if blank}
              <div class="h-full flex justify-center items-center">
                <img src={InterfaceBlank} height="142" width="auto" alt="Blank Interface" />
              </div>
            {:else if image}
              <div class="h-full flex flex-col justify-center items-center">
                {#if imgUpload}
                  <img class="imgUpload" src={imgUpload} alt="d" />
                {:else}
                  <img class="imgUpload opacity-20" src={ImgPlaceholder} alt="placehoder" />
                {/if}
                <IconButton
                  size={50}
                  on:click={() => {
                    fileinput.click();
                  }}
                  class=""
                >
                  <Icon
                    name="image-upload"
                    class="cursor-pointer"
                    size={40}
                    iconColor="text-body dark:text-body-dark"
                    pathName={ImageUpload}
                    viewBox="0 0 20 20"
                  />
                </IconButton>
                <div
                  class="chan text-center relative"
                  on:click={() => {
                    fileinput.click();
                  }}
                  on:keydown
                >
                  Choose Image
                </div>
                <input
                  style="display:none"
                  type="file"
                  accept=".jpg, .jpeg, .png"
                  on:change={(e) => onFileSelected(e)}
                  bind:this={fileinput}
                />
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
                on:click={() => (hiddenRight = true)}>cancel</button
              >
              <Button color="default" size="sm" label="Save" on:click class="" />
            </div>
          </div>
        </div></span
      >
    {/if}
  </aside>
</Drawer>

<Modal bind:open={deleteModal} size="xs" autoclose title="Delete Event Modal">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="warning" pathName={Warning} class="mr-1" />
    <span class="whitespace-nowrap text-sm text-body"
      >Are you sure you want to delete this Event Modal?</span
    >
  </div>
  <div class="my-3 text-center w-full inline-flex justify-center items-center p px-6">
    {#if done}
      <Icon
        name="checkmark"
        iconColor="text-green"
        class="mr-1"
        size={12}
        pathName={Checkmark}
        viewBox="0 0 48 48"
      />
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

<Modal bind:open={importModal} size="xs" autoclose title="Import JSON File">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <Icon name="file" pathName={FileIcon} class="mr-1" />
    <ProgressBar done />
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
    <button
      class="text-sm text-focus hover:text-[#054FDE] transition duration-200 ease-in underline"
      on:click={hide}
      on:click={() => (input.value = '')}>Cancel</button
    >
    <Button input boundTo="submit-form" tabindex={0} color="default" size="sm" label="Done" />
  </div>
</Modal>

<Grid
  {decider}
  default_audience_placements={$grid?.default_audience}
  audiences={$grid?.audiences}
  timeline_placements={$grid?.timeline}
  streams={$grid?.streams}
  default_stream_placements={$grid?.default_stream}
  flows={$grid?.flows}
  column_count={$grid?.column_count || 0}
/>

<slot />

<style>
  .imgUpload {
    display: flex;
    height: 100%;
    width: 100%;
  }
</style>
