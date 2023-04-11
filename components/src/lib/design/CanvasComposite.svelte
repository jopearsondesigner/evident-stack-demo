<script>
  import 'components/src/app.css';
  import { Accordion, AccordionItem } from 'svelte-accessible-accordion';
  import Grid from '$lib/design/Grid.svelte';
  import Drawer from '$lib/drawer/Drawer.svelte';
  import Sidebar from '$lib/drawer/Sidebar.svelte';
  import SidebarWrapper from '$lib/drawer/SidebarWrapper.svelte';
  import SidebarContainer from '$lib/drawer/SidebarContainer.svelte';
  import SidebarGroup from '$lib/drawer/SidebarGroup.svelte';
  import SidebarDropdownWrapper from '$lib/drawer/SidebarDropdownWrapper.svelte';
  import SidebarDropdownItem from '$lib/drawer/SidebarDropdownItem.svelte';
  import SidebarItem from '$lib/drawer/SidebarItem.svelte';
  import Textarea from '$lib/form/Textarea.svelte';
  import Label from '$lib/form/Label.svelte';
  let code = '"domain" {\n' + '\tfoo: string, \n' + '\tbar: string, \n' + '\tbaz: int\n' + '}';
  import Schema from '$lib/icons/Schema.svelte';
  import ThemeSwitch from '$lib/utils/ThemeSwitch.svelte';
  import Navbar from '$lib/navbar/Navbar.svelte';
  import NavHamburger from '$lib/navbar/NavHamburger.svelte';
  import NavBrand from '$lib/navbar/NavBrand.svelte';
  import NavInner from '$lib/navbar/NavInner.svelte';
  import NavToolbar from '$lib/navbar/NavToolbar.svelte';
  import Icon from '$lib/Icon.svelte';
  import Button from '$lib/Button.svelte';
  import Download from '$lib/icons/Download.svelte';
  import IconButton from '$lib/IconButton.svelte';
  import Avatar from '$lib/Avatar.svelte';
  import Docs from '$lib/icons/Docs.svelte';
  import Support from '$lib/icons/Support.svelte';
  import A from '$lib/typography/A.svelte';
  import MaybeTooltip from '$lib/utils/MaybeTooltip.svelte';
  import Logo from '$lib/assets/images/global/evidentStackLogo.svg';
  import DesignLogo from '$lib/assets/images/product/design/evidentDesignLogo.svg';
  import DataLogo from '$lib/assets/images/product/data/evidentDataLogo.svg';
  import DomainFunctionsLogo from '$lib/assets/images/product/DomainFunctions/evidentDomainFunctionsLogo.svg';
  import DeployLogo from '$lib/assets/images/product/deploy/evidentDeployLogo.svg';
  import DatabaseLogo from '$lib/assets/images/product/database/evidentDatabaseLogo.svg';
  import EventIcon from '$lib/icons/EventIcon.svelte';
  import CommandIcon from '$lib/icons/CommandIcon.svelte';
  import ReadModelIcon from '$lib/icons/ReadModelIcon.svelte';
  let event = true;
  let command = false;
  let readModel = false;
  let isClosed = true;
  export let user = true;
  let hidden = true;
  let hiddenRight = true;
  let handleDrawer = () => {
    hidden = !hidden;
  };
  let handleClick = () => {
    isClosed = !isClosed;
  };
  let website = false;
  const hide = (e) => {
    e.preventDefault();
    isClosed = true;
  };
  /**
   * @type {boolean}
   */
  let expanded = true;
</script>

<Navbar>
  <NavInner navDivClass="flex justify-between items-center">
    <NavToolbar navClass="px-3 h-9 inline-flex space-x-4 mx-3 items-center">
      <NavHamburger {website} on:click={() => handleDrawer()} on:click={() => (isClosed = true)} />
      <NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3 cursor-default" />
    </NavToolbar>
    <NavToolbar
      navClass="px-3 h-9 inline-flex space-x-2 mx-3 items-center border-l border-gray-secondary dark:border-border-dark"
    >
      {#if user}
        <MaybeTooltip tip="Docs">
          <IconButton
            ><Icon
              name="docs"
              size={18}
              iconColor="text-body-light dark:text-body-dark"
              pathName={Docs}
            />
            ></IconButton
          >
        </MaybeTooltip>
        <MaybeTooltip tip="Support">
          <IconButton
            ><Icon
              name="support"
              size={18}
              iconColor="text-body-light dark:text-body-dark"
              pathName={Support}
            /></IconButton
          >
        </MaybeTooltip>
        <span class="flex justify-center px-3">
          <Avatar />
        </span>
      {:else}
        <A class="text-sm mr-3 transition duration-300 ease-in">Log in</A>
        <Button gradient color="brandStackPrimary" size="sm" on:click label="Sign up" />
      {/if}
    </NavToolbar>
  </NavInner>
</Navbar>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16"><ThemeSwitch /></span>
<Drawer {hidden} className="mt-4">
  <Sidebar class={!isClosed ? 'w-[417px]' : 'w-[240px]'} {isClosed}>
    <SidebarWrapper>
      <Accordion>
        <SidebarContainer src={DesignLogo} title="Design" id="design" bind:expanded>
          <SidebarGroup>
            <SidebarDropdownWrapper label="	Schema" on:click={() => handleClick()}>
              <Icon
                slot="icon"
                name="schema"
                size={16}
                iconColor="fill-current text-gray-brand-4 dark:text-white transition duration-200 ease-in"
                ><Schema /></Icon
              >
              <Icon
                slot="icon-open"
                name="schema"
                size={16}
                iconColor="fill-current text-white dark:text-white transition duration-200 ease-in"
                ><Schema /></Icon
              >
              <SidebarDropdownItem feature>
                <Label class="mt-2 mb-6" color="default"
                  ><span class="text-body dark:text-white">Event Model Schema</span>
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
                <div class="mt-6 mx-3 space-x-3 flex justify-end">
                  <button
                    class="text-sm underline text-focus dark:text-white hover:text-[#054FDE] dark:hover:text-focus transition duration-200 ease-in"
                    on:click>cancel</button
                  >
                  <Button color="default" size="sm" label="Edit" on:click class="" />
                </div>
              </SidebarDropdownItem>
            </SidebarDropdownWrapper>
            <SidebarItem blank>
              <Button
                label="Export"
                gradient
                color="ghost"
                size="sm"
                className="my-4"
                on:click
                class=""
                ><Icon
                  slot="icon"
                  name="download"
                  size={12}
                  iconColor="text-body-light dark:text-white"
                  class="inline-flex mb-1"><Download /></Icon
                ></Button
              >
            </SidebarItem>
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer src={DataLogo} id="data" title="Data" on:click={() => (isClosed = true)}>
          <SidebarGroup>
            <SidebarItem label="Hello, please develop me!" blank />
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DomainFunctionsLogo}
          id="domain-functions"
          title="Domain Functions"
          on:click={() => (isClosed = true)}
        >
          <SidebarGroup>
            <SidebarItem label="Hello, please develop me!" blank />
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DeployLogo}
          id="deploy"
          title="Deploy"
          on:click={() => (isClosed = true)}
        >
          <SidebarGroup>
            <SidebarItem label="Hello, please develop me!" blank />
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DatabaseLogo}
          id="database"
          title="Database"
          on:click={() => (isClosed = true)}
        >
          <SidebarGroup>
            <SidebarItem label="Hello, please develop me!" blank />
          </SidebarGroup>
        </SidebarContainer>
      </Accordion>
    </SidebarWrapper>
  </Sidebar>
</Drawer>

<div class="text-center z-[31] absolute inset-x-0 pt-4 mt-16">
  <!--For testing-->
  <Button gradient color="primary" size="sm" on:click={() => (hiddenRight = false)} class=""
    >Show Sidebar</Button
  ><br />
  <button on:click={() => (hiddenRight = true)} class="mt-4">Close</button>
</div>

<!-- backdrop -->

<Drawer placement="right" className="mt-4" bind:hidden={hiddenRight} drawerRight>
  <aside
    class="w-[480px] h-full flex items-center px-6 bg-white dark:bg-dark-2 border-l border-gray-primary dark:border-gray-brand-3"
  >
    <span class="w-full">
      <h3 class="text-left text-default font-extrabold text-body-light dark:text-body-dark mb-1">
        Placement Details
      </h3>
      <div class="w-full p-6 border rounded border-border-light dark:border-border-dark">
        <div class="inline-flex">
          {#if event}
            <Icon name="event-icon" size={48} class="" pathName={EventIcon} />
          {:else if command}
            <Icon name="command-icon" size={48} class="" pathName={CommandIcon} />
          {:else if readModel}
            <Icon name="read-model-icon" size={48} class="" pathName={ReadModelIcon} />
          {/if}
          <h2 class="ml-3 self-end text-xl font-bold text-body-light dark:text-body-dark">
            Component Name
          </h2>
        </div>
        <p class="my-3 text-sm leading-normal text-body dark:text-white">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt
          ut labore et dolore magna aliqua.
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
            on:click>cancel</button
          >
          <Button color="default" size="sm" label="Edit" on:click class="" />
        </div>
      </div>
    </span>
  </aside>
</Drawer>

<main
  class="left-0 relative transition-all duration-[200ms] ml-0"
  class:left-60={!hidden}
  class:ml-[177px]={!isClosed}
  class:ease-in={!isClosed}
>
  <!-- <Grid
    default_audience_placements={[
      ,
      ,
      { id: '1', name: 'An interface!', description: 'blah *blah* **blah**' },
      ,
      ,
      ,
      ,
      ,
      ,
      ,
      ,
      { id: '2', name: 'Another interface!', description: 'blah *blah* **blah**' }
    ]}
    audiences={[
      {
        name: 'A named audience',
        placements: [
          ,
          ,
          ,
          { id: '3', name: 'An interface!', description: 'blah *blah* **blah**' },
          ,
          { id: '4', name: 'Another interface!', description: 'blah *blah* **blah**' }
        ]
      },
      {
        name: 'Another named audience',
        placements: [
          ,
          { id: '5', name: 'An interface!', description: 'blah *blah* **blah**' },
          ,
          ,
          ,
          { id: '6', name: 'Another interface!', description: 'blah *blah* **blah**' }
        ]
      }
    ]}
    timeline_placements={[
      ,
      { id: '5', name: 'A command!', type: 'command', description: 'blah *blah* **blah**' },
      ,
      { id: '6', name: 'A read model!', type: 'readModel', description: 'blah *blah* **blah**' },
      ,
      { id: '7', name: 'Another command!', type: 'command', description: 'blah *blah* **blah**' },
      {
        id: '8',
        name: 'Another read model!',
        type: 'readModel',
        description: 'blah *blah* **blah**'
      }
    ]}
    streams={[
      {
        name: 'A named stream',
        placements: [
          ,
          ,
          ,
          { id: '9', name: 'An event!', description: 'blah *blah* **blah**' },
          ,
          ,
          ,
          ,
          ,
          ,
          ,
          ,
          ,
          { id: '10', name: 'Another event!', description: 'blah *blah* **blah**' }
        ]
      }
    ]}
    default_stream_placements={[
      ,
      ,
      { id: '11', name: 'An event!', description: 'blah *blah* **blah**' },
      ,
      ,
      { id: '12', name: 'Another event!', description: 'blah *blah* **blah**' }
    ]}
    flows={[]}
  /> -->
  <Grid />
</main>

<style>
  :global([data-accordion]) {
    height: 100%;
  }
</style>
