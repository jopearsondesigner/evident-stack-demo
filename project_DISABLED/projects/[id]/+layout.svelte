<script lang="ts">
  import classNames from 'classnames';
  import type { LayoutData } from './$types';
  import { page } from '$app/stores';

  import Navbar from '$components/navbar/Navbar.svelte';
  import NavInner from '$components/navbar/NavInner.svelte';
  import NavToolbar from '$components/navbar/NavToolbar.svelte';
  import NavHamburger from '$components/navbar/NavHamburger.svelte';
  import NavBrand from '$components/navbar/NavBrand.svelte';
  import Button from '$components/Button.svelte';
  import IconButton from '$components/IconButton.svelte';
  import MaybeTooltip from '$components/utils/MaybeTooltip.svelte';
  import Icon from '$components/Icon.svelte';
  import Home from '$components/icons/Home.svelte';
  import Docs from '$components/icons/Docs.svelte';
  import Support from '$components/icons/Support.svelte';
  import Profile from '$components/icons/Profile.svelte';
  import Logo from '$components/assets/images/global/evidentStackLogo.svg';

  import ThemeSwitch from '$components/utils/ThemeSwitch.svelte';

  import Drawer from '$components/drawer/Drawer.svelte';
  import DrawerDetails from '$components/drawer/DrawerDetails.svelte';
  import Sidebar from '$components/drawer/Sidebar.svelte';
  import SidebarContainer from '$components/drawer/SidebarContainer.svelte';
  import SidebarGroup from '$components/drawer/SidebarGroup.svelte';
  import { Accordion } from 'svelte-accessible-accordion';
  import SidebarItem from '$components/drawer/SidebarItem.svelte';
  import Download from '$components/icons/Download.svelte';
  import DesignLogo from '$components/assets/images/product/design/evidentDesignLogo.svg';
  import DataLogo from '$components/assets/images/product/data/evidentDataLogo.svg';
  import DomainFunctionsLogo from '$components/assets/images/product/domainFunctions/evidentDomainFunctionsLogo.svg';
  import DeployLogo from '$components/assets/images/product/deploy/evidentDeployLogo.svg';
  import DatabaseLogo from '$components/assets/images/product/database/evidentDatabaseLogo.svg';
  import DropdownMenu from '$components/dropdown/DropdownMenu.svelte';
  import DropdownItem from '$components/dropdown/DropdownItem.svelte';
  import DropdownDivider from '$components/dropdown/DropdownDivider.svelte';

  import TreeView from '$components/data/tree/TreeView.svelte';
  import TreeItem from '$components/data/tree/TreeItem.svelte';
  // let href = `/projects/${$page.params.id}/data/schemas/`;

  import Pin from '$components/icons/Pin.svelte';
  import EventIcon from '$components/icons/EventIcon.svelte';
  import CommandIcon from '$components/icons/CommandIcon.svelte';
  import ReadModelIcon from '$components/icons/ReadModelIcon.svelte';
  import InterfaceIcon from '$components/icons/InterfaceIcon.svelte';
  import EventModelIcon from '$components/icons/EventModel.svelte';
  let isActive: string | number;
  let btnClass: string | undefined = '';

  // Data for testing
  const tree_data = [
    {
      name: 'Autonomo Mobile iOS App',
      type: 'event-model',
      id: 1,
      children: [
        {
          name: 'My Vehicles',
          type: 'read-model',
          id: 2,
          children: [
            {
              name: 'Vehicle Added',
              type: 'event',
              id: 3
            },
            {
              name: 'Placement',
              type: 'placement',
              id: 4
            },
            {
              name: 'Change',
              type: 'event',
              id: 5
            },
            {
              name: 'Placement',
              type: 'placement',
              id: 6
            },
            {
              name: 'Vehicle Removed',
              type: 'event',
              id: 7
            },
            {
              name: 'Placement',
              type: 'placement',
              id: 8
            },
            {
              name: 'Add a Vehicle',
              type: 'interface',
              id: 9
            },
            {
              name: 'Placement',
              type: 'placement',
              id: 10
            },
            {
              name: 'My Vehicles',
              type: 'interface',
              id: 11
            },
            {
              name: 'Placement',
              type: 'placement',
              id: 12
            },
            {
              name: 'Remove a Vehicle',
              type: 'interface',
              id: 13
            },
            {
              name: 'Placement',
              type: 'placement',
              id: 14
            }
          ]
        },
        {
          name: 'Add a Vehicle',
          type: 'command',
          id: 15,
          children: [
            {
              name: 'Vehicle Added',
              type: 'event',
              id: 16
            },
            {
              name: 'Placement',
              type: 'placement',
              id: 17
            },
            {
              name: 'Add a Vehicle',
              type: 'interface',
              id: 18
            },
            {
              name: 'Placement',
              type: 'placement',
              id: 19
            }
          ]
        }
      ]
    }
  ];

  import type { Readable } from 'svelte/store';
  import { onMount } from 'svelte';
  import { loadSyncWorker } from '$lib/state/sync';
  import { goto } from '$app/navigation';

  export let data: LayoutData;

  // TODO: this shouldn't be grid, but rather another read model
  const { decider, grid, session, supabase } = data;

  // ==== Model Sync State

  let sync_status: Readable<number>;

  onMount(async () => {
    sync_status = await loadSyncWorker(session!);
  });

  // ==== end Model Sync State

  // === Authentication

  const handleSignOut = async () => {
    await supabase.auth.signOut();
    await goto('/auth');
  };

  // === End Authentication

  // === left-nav state

  $: expandedLeftNavItem = $page.data.product;

  $: designExpanded = expandedLeftNavItem == 'design';
  $: dataExpanded = expandedLeftNavItem == 'data';
  $: domainFunctionsExpanded = expandedLeftNavItem == 'domain-functions';
  $: deployExpanded = expandedLeftNavItem == 'deploy';
  $: dbExpanded = expandedLeftNavItem == 'db';

  let leftNavOpen = false;
  let hidden = false;
  let toggleLeftNav = () => {
    hidden = !hidden;
    leftNavOpen = !leftNavOpen;
  };

  // == end left-nav state
</script>

<!-- TODO: don't use `grid` here -->
<svelte:head>
  <title>{$grid?.name ?? 'Project'} | Evident Stack</title>
</svelte:head>

<div class="min-h-screen">
  <Navbar website={false}>
    <NavInner navDivClass="flex justify-between items-center">
      <NavToolbar navClass="px-3 mx-3 h-9 inline-flex space-x-4 items-center">
        <NavHamburger website={false} hamburgerClass="mx-2" on:click={toggleLeftNav} />
        <NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3 cursor-default" />
        <div
          class="h-9 pr-3 border-r border-gray-secondary dark:border-border-dark flex items-center"
        >
          <MaybeTooltip tip="Home" position="tooltip-bottom">
            <IconButton href="/">
              <Icon
                name="home"
                size={16}
                iconColor="text-body-light dark:text-body-dark"
                pathName={Home}
              />
            </IconButton>
          </MaybeTooltip>
        </div>
      </NavToolbar>
      <NavToolbar
        navClass="px-3 h-9 inline-flex space-x-2.5 mx-3 items-center border-l border-gray-secondary dark:border-border-dark"
      >
        {#if session?.user}
          <DropdownMenu product={true} name="profile" marginTop="mt-9">
            <IconButton slot="button" margin="mx-2">
              <Icon
                name="profile"
                size={28}
                viewBox="0 0 32 32"
                class="vertical-middle"
                iconColor=""
                pathName={Profile}
              />
            </IconButton>
            <DropdownItem padding="pt-2 pb-4 px-4">
              {session.user.email ?? 'email@example.com'}
            </DropdownItem>
            <DropdownItem href="/account" textOnly={true}>Account</DropdownItem>
            <DropdownDivider />
            <DropdownItem className="flex-1 text-center" textOnly={true}>
              <button class="button inline" on:click|preventDefault={handleSignOut}>Sign Out</button
              >
            </DropdownItem>
          </DropdownMenu>

          <MaybeTooltip tip="Docs" position="tooltip-bottom">
            <IconButton>
              <Icon
                name="docs"
                size={18}
                iconColor="text-body-light dark:text-body-dark"
                pathName={Docs}
              />
            </IconButton>
          </MaybeTooltip>
          <MaybeTooltip tip="Support" position="tooltip-bottom">
            <IconButton>
              <Icon
                name="support"
                size={18}
                iconColor="text-body-light dark:text-body-dark"
                pathName={Support}
              />
            </IconButton>
          </MaybeTooltip>
        {/if}
      </NavToolbar>
    </NavInner>
  </Navbar>

  <span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16"><ThemeSwitch /></span>

  <Drawer placement="left" bind:hidden>
    <!-- TODO: don't use `grid` here -->
    <Sidebar class="w-[240px]">
      <DrawerDetails
        name={$grid?.name ?? 'Project Name'}
        description={$grid?.description ?? 'Project Description'}
        sync_status={$sync_status}
        isOpen={false}
      />
      <Accordion>
        <SidebarContainer
          src={DesignLogo}
          href="/projects/{$page.params.id}/design"
          id="design"
          title="Design"
          bind:expanded={designExpanded}
        >
          <SidebarGroup>
            <SidebarItem maxHeightNum={194} blank>
              <Button
                label="Export JSON"
                gradient
                color="ghost"
                size="sm"
                className="my-4"
                on:click={decider.export_json}
              >
                <Icon
                  slot="icon"
                  name="download"
                  size={12}
                  iconColor="text-body-light dark:text-white"
                  class="inline-flex mb-1"
                  pathName={Download}
                />
              </Button>
            </SidebarItem>
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DataLogo}
          href="/projects/{$page.params.id}/data/schemas/[schema]"
          id="data"
          title="Data"
          bind:expanded={dataExpanded}
        >
          <SidebarGroup>
            <SidebarItem padding="p-0" maxHeightNum={322} blank>
              <TreeView {tree_data} let:item bind:isActive>
                {#if item.children}
                  <TreeItem
                    elementClass="flex items-center w-full group h-7"
                    href="/projects/{$page.params.id}/data/schemas/{item.id}"
                  >
                    <div class={classNames('')}>
                      {#if item.type === 'event-model'}
                        <Icon
                          name="{item.type}-icon"
                          size={14}
                          iconColor="fill-current"
                          class="inline-flex ml-1"
                          pathName={EventModelIcon}
                        />
                      {:else if item.type === 'command'}
                        <Icon
                          name="{item.type}-icon"
                          size={14}
                          class="inline-flex ml-1"
                          viewBox="0 0 20 20"
                          pathName={CommandIcon}
                        />
                      {:else if item.type === 'read-model'}
                        <Icon
                          name="{item.type}-icon"
                          size={14}
                          class="inline-flex ml-1"
                          viewBox="0 0 20 20"
                          pathName={ReadModelIcon}
                        />
                      {/if}
                      <span class="ml-0.5">{item.name}</span>
                    </div>
                  </TreeItem>
                {:else}
                  <TreeItem
                    elementClass={classNames(
                      btnClass,
                      'pl-[35px] flex items-center w-full h-7 text-body dark:text-body-dark text-default bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20]'
                    )}
                    href="/projects/{$page.params.id}/data/schemas/{item.id}"
                  >
                    {#if item.type == 'placement'}
                      <span
                        class="placement-indicator relative border-l border-b border-gray-brand-2 dark:border-gray-brand-2 w-1.5 h-5 -top-[9px] -right-[9px] mr-2.5"
                      />
                    {/if}
                    {#if item.type === 'placement'}
                      <Icon
                        name={item.type}
                        size={12}
                        class="inline-flex"
                        viewBox="0 0 20 20"
                        iconColor="fill-current"
                        pathName={Pin}
                      />
                    {:else if item.type === 'event-model'}
                      <Icon
                        name="{item.type}-icon"
                        size={14}
                        class="inline-flex ml-1"
                        viewBox="0 0 20 20"
                        pathName={EventModelIcon}
                      />
                    {:else if item.type === 'event'}
                      <Icon
                        name="{item.type}-icon"
                        size={14}
                        class="inline-flex ml-1"
                        viewBox="0 0 20 20"
                        pathName={EventIcon}
                      />
                    {:else if item.type === 'interface'}
                      <Icon
                        name="{item.type}-icon"
                        size={14}
                        class="inline-flex ml-1"
                        viewBox="0 0 20 20"
                        pathName={InterfaceIcon}
                      />
                    {/if}
                    <span class="ml-1">{item.name}</span>
                  </TreeItem>
                {/if}
              </TreeView>
            </SidebarItem>
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DomainFunctionsLogo}
          href="/projects/{$page.params.id}/domain-functions"
          id="domain-functions"
          title="Domain Functions"
          bind:expanded={domainFunctionsExpanded}
        >
          <SidebarGroup />
        </SidebarContainer>
        <SidebarContainer
          src={DeployLogo}
          href="/projects/{$page.params.id}/deploy"
          id="deploy"
          title="Deploy"
          bind:expanded={deployExpanded}
        >
          <SidebarGroup />
        </SidebarContainer>
        <SidebarContainer
          src={DatabaseLogo}
          href="/projects/{$page.params.id}/db"
          id="db"
          title="Database"
          bind:expanded={dbExpanded}
        >
          <SidebarGroup />
        </SidebarContainer>
      </Accordion>
    </Sidebar>
  </Drawer>

  <main
    class="relative left-0 right-0 transition-all duration-[200ms] pt-16 ml-0"
    class:ml-[240px]={!leftNavOpen}
    class:ease-out={leftNavOpen}
    class:duration-100={leftNavOpen}
  >
    <slot />
  </main>
</div>

<style>
  .selected {
    background-color: #1e6aff !important;
    color: white !important;
  }
  .selected *,
  .selected button {
    color: white !important;
  }
</style>
