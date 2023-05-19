<script lang="ts">
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
  import Sidebar from '$components/drawer/Sidebar.svelte';
  import SidebarContainer from '$components/drawer/SidebarContainer.svelte';
  import SidebarGroup from '$components/drawer/SidebarGroup.svelte';
  import SidebarDropdownWrapper from '$components/drawer/SidebarDropdownWrapper.svelte';
  import { Accordion, AccordionItem } from 'svelte-accessible-accordion';
  import SidebarDropdownItem from '$components/drawer/SidebarDropdownItem.svelte';
  import SidebarItem from '$components/drawer/SidebarItem.svelte';
  import Textarea from '$components/form/Textarea.svelte';
  import Label from '$components/form/Label.svelte';
  import Schema from '$components/icons/Schema.svelte';
  import Download from '$components/icons/Download.svelte';
  import DesignLogo from '$components/assets/images/product/design/evidentDesignLogo.svg';
  import DataLogo from '$components/assets/images/product/data/evidentDataLogo.svg';
  import DomainFunctionsLogo from '$components/assets/images/product/domainFunctions/evidentDomainFunctionsLogo.svg';
  import DeployLogo from '$components/assets/images/product/deploy/evidentDeployLogo.svg';
  import DatabaseLogo from '$components/assets/images/product/database/evidentDatabaseLogo.svg';
  import DropdownMenu from '$components/dropdown/DropdownMenu.svelte';
  import DropdownItem from '$components/dropdown/DropdownItem.svelte';
  import DropdownDivider from '$components/dropdown/DropdownDivider.svelte';
  import { enhance } from '$app/forms';

  // === Authentication

  export let data: LayoutData;

  $: ({ session, grid } = data)

  let signOutLoading = false;

  function handleSignOutSubmit() {
    signOutLoading = true;
    return async () => {
      signOutLoading = false;
    };
  }

  // === End Authentication

  // === left-nav state

  $: expandedLeftNavItem = $page.data.product;

  let leftNavOpen = true;

  $: designExpanded = expandedLeftNavItem == 'design';
  $: dataExpanded = expandedLeftNavItem == 'data';
  $: domainFunctionsExpanded = expandedLeftNavItem == 'domain-functions';
  $: deployExpanded = expandedLeftNavItem == 'deploy';
  $: dbExpanded = expandedLeftNavItem == 'db';

  const toggleLeftNav = () => {
    leftNavOpen = !leftNavOpen;
  };

  // == end left-nav state
</script>

<div class="min-h-screen">
  <Navbar website={false}>
    <NavInner navDivClass="flex justify-between items-center">
      <NavToolbar navClass="px-3 mx-3 h-9 inline-flex space-x-4 items-center">
        <NavHamburger
          website={false}
          hamburgerClass="mx-2"
          on:click={toggleLeftNav} />
        <NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3 cursor-default" />
        <div class="h-9 pr-3 border-r border-gray-secondary dark:border-border-dark flex items-center">
          <MaybeTooltip tip="Home" position="tooltip-bottom">
            <IconButton href="/">
              <Icon
                name="home"
                size={16}
                iconColor="text-body-light dark:text-body-dark"
                pathName={Home} />
            </IconButton>
          </MaybeTooltip>
        </div>
      </NavToolbar>
      <NavToolbar
        navClass="px-3 h-9 inline-flex space-x-2.5 mx-3 items-center border-l border-gray-secondary dark:border-border-dark">
        {#if session?.user}
          <DropdownMenu product={true} name="profile" marginTop="mt-9">
            <IconButton slot="button" margin="mx-2 mt-1">
              <Icon
                name="profile"
                size={32}
                viewBox="0 0 32 32"
                class="vertical-middle"
                iconColor=""
                pathName={Profile} />
            </IconButton>
            <DropdownItem padding="pt-2 pb-4 px-4" textOnly={true}>
              {session.user.email ?? "email@example.com"}
            </DropdownItem>
            <DropdownItem href="/account">Account</DropdownItem>
            <DropdownDivider />
            <DropdownItem className="flex-1">
              <form method="POST" action="/auth/sign-out" use:enhance={handleSignOutSubmit}>
                <button class="button inline" disabled={signOutLoading}>sign out</button>
              </form>
            </DropdownItem>
          </DropdownMenu>

          <MaybeTooltip tip="Docs" position="tooltip-bottom">
            <IconButton>
              <Icon
                name="docs"
                size={18}
                iconColor="text-body-light dark:text-body-dark"
                pathName={Docs} />
            </IconButton>
          </MaybeTooltip>
          <MaybeTooltip tip="Support" position="tooltip-bottom">
            <IconButton>
              <Icon
                name="support"
                size={18}
                iconColor="text-body-light dark:text-body-dark"
                pathName={Support} />
            </IconButton>
          </MaybeTooltip>
        {:else}
          <Button
            href="/auth/sign-in"
            gradient
            color="brandStackPrimary"
            size="sm"
            on:click
            label="Sign In"
            />
          {/if}
        </NavToolbar>
    </NavInner>
  </Navbar>

  <span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16"><ThemeSwitch /></span>

  <Drawer placement="left" hidden={false}>
    <Sidebar name={$grid?.name ?? "Project Name"}
             description={$grid?.description ?? "Project Description"}
             class={leftNavOpen ? 'w-[417px]' : 'w-[160px]'}>
      <Accordion class="grow-1 flex flex-col">
        <SidebarContainer
          src={DesignLogo}
          href="/projects/{$page.params.id}/design"
          id="design"
          title="Design"
          bind:expanded={designExpanded}>
          <SidebarGroup>
            <SidebarDropdownWrapper label="Schema">
              <Icon
                slot="icon"
                name="schema"
                size={16}
                iconColor="fill-current text-gray-brand-4 dark:text-white transition duration-200 ease-in"
                pathName={Schema} />
              <Icon
                slot="icon-open"
                name="schema"
                size={16}
                iconColor="fill-current text-white dark:text-white transition duration-200 ease-in"
                pathName={Schema} />
              <SidebarDropdownItem feature>
                <Label class="mt-2 mb-6" color="default">
                  <span class="text-body dark:text-white">Event Model Schema</span>
                  <Textarea
                    placeholder=""
                    value=""
                    name="description"
                    rows="6"
                    class="mt-1 font-mono block w-full overflow-auto text-sm border border-border-light dark:border-border-dark px-10 py-2.5"
                    style="background-color: rgba(48, 56, 65, 100%); color: #D8DEE9;"
                    disabled />
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
                class="">
                <Icon
                  slot="icon"
                  name="download"
                  size={12}
                  iconColor="text-body-light dark:text-white"
                  class="inline-flex mb-1"
                  pathName={Download} />
              </Button>
            </SidebarItem>
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DataLogo}
          href="/projects/{$page.params.id}/data"
          id="data"
          title="Data"
          bind:expanded={dataExpanded}>
          <SidebarGroup>
            <SidebarItem label="Hello, please develop me!" blank />
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DomainFunctionsLogo}
          href="/projects/{$page.params.id}/domain-functions"
          id="domain-functions"
          title="Domain Functions"
          bind:expanded={domainFunctionsExpanded} >
          <SidebarGroup />
        </SidebarContainer>
        <SidebarContainer
          src={DeployLogo}
          href="/projects/{$page.params.id}/deploy"
          id="deploy"
          title="Deploy"
          bind:expanded={deployExpanded} >
          <SidebarGroup />
        </SidebarContainer>
        <SidebarContainer
          src={DatabaseLogo}
          href="/projects/{$page.params.id}/db"
          id="db"
          title="Database"
          bind:expanded={dbExpanded} >
          <SidebarGroup />
        </SidebarContainer>
      </Accordion>
    </Sidebar>
  </Drawer>

  <main
    class="relative left-0 right-0 transition-all duration-[200ms] pt-16"
    class:ml-[160px]={!leftNavOpen}
    class:ml-[417px]={leftNavOpen}
    class:ease-in={leftNavOpen} >
    <slot />
  </main>
</div>
