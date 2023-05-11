<script lang="ts">
  import CodeMirror from 'svelte-codemirror-editor';
  import { javascript } from '@codemirror/lang-javascript';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { page } from '$app/stores';
  import Icon from '$components/Icon.svelte';
  import Sidebar from '$components/drawer/Sidebar.svelte';
  import SidebarWrapper from '$components/drawer/SidebarWrapper.svelte';
  import SidebarContainer from '$components/drawer/SidebarContainer.svelte';
  import SidebarGroup from '$components/drawer/SidebarGroup.svelte';
  import SidebarDropdownWrapper from '$components/drawer/SidebarDropdownWrapper.svelte';
  import SidebarDropdownItem from '$components/drawer/SidebarDropdownItem.svelte';
  import SidebarItem from '$components/drawer/SidebarItem.svelte';
  import Textarea from '$components/form/Textarea.svelte';
  import Label from '$components/form/Label.svelte';
  import Schema from '$components/icons/Schema.svelte';
  import Download from '$components/icons/Download.svelte';
  import DesignLogo from '$components/assets/images/product/design/evidentDesignLogo.svg';
  import Button from '$components/Button.svelte';

  let isVerticalOpen = false;
  let isClosed = true;
  let valueEventModel = '';
  let grid = true;
  let designExpanded = false;
  let path: string;

  $: path = $page.url.pathname;
  console.log(path);

  $: if (path == '/design/models') {
    designExpanded = true;
    console.log(path);
    console.log(designExpanded);
  } else {
    designExpanded = false;
  }

  let handleClick = () => {
    isClosed = !isClosed;
  };
</script>

<SidebarContainer
  src={DesignLogo}
  title="Design"
  id="design/models"
  slot="design"
  on:click={() => (grid = true)}
  bind:expanded={designExpanded}
>
  <SidebarGroup>
    <SidebarDropdownWrapper label="Schema" on:click={() => handleClick()} bind:isVerticalOpen>
      <Icon
        slot="icon"
        name="schema"
        size={16}
        iconColor="fill-current text-gray-brand-4 dark:text-white transition duration-200 ease-in"
        pathName={Schema}
      />
      <Icon
        slot="icon-open"
        name="schema"
        size={16}
        iconColor="fill-current text-white dark:text-white transition duration-200 ease-in"
        pathName={Schema}
      />
      <SidebarDropdownItem feature>
        <Label class="mt-8"
          ><span class="mb-1 text-body dark:text-white">Event Model Schema</span></Label
        >
        <CodeMirror
          value={valueEventModel}
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
        <div class="mt-6 mx-3 space-x-3 flex justify-end">
          <button
            class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
            on:click={() => handleClick()}
            on:click={() => (isVerticalOpen = false)}>cancel</button
          >
          <Button color="default" size="sm" label="Save" on:click class="" />
        </div>
      </SidebarDropdownItem>
    </SidebarDropdownWrapper>
    <SidebarItem blank>
      <Button label="Export" gradient color="ghost" size="sm" className="my-4" on:click class=""
        ><Icon
          slot="icon"
          name="download"
          size={12}
          iconColor="text-body-light dark:text-white"
          class="inline-flex mb-1"
          pathName={Download}
        /></Button
      >
    </SidebarItem>
  </SidebarGroup>
</SidebarContainer>
