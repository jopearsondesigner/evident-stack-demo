<script lang="ts">
  import '../app.css';
  import type { LayoutData } from './$types';
  import { onMount } from 'svelte';
  import { invalidate } from '$app/navigation';
  import { handleSignOut } from '$lib/user';
  import { goto } from '$app/navigation';

  import CodeMirror from 'svelte-codemirror-editor';
  import { javascript } from '@codemirror/lang-javascript';
  import { oneDark } from '@codemirror/theme-one-dark';
  let valueEventModel = '';

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
  import AdminPortal from '$components/icons/AdminPortal.svelte';
  import Profile from '$components/icons/Profile.svelte';
  import Logo from '$components/assets/images/global/evidentStackLogo.svg';

  import ThemeSwitch from '$components/utils/ThemeSwitch.svelte';

  import Drawer from '$components/drawer/Drawer.svelte';
  import Sidebar from '$components/drawer/Sidebar.svelte';
  import SidebarWrapper from '$components/drawer/SidebarWrapper.svelte';
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
  import DataGraphic from '$components/assets/images/product/global/DataGraphic.svg';
  import DomainFunctionsGraphic from '$components/assets/images/product/global/DomainFunctionsGraphic.svg';
  import DeployGraphic from '$components/assets/images/product/global/DeployGraphic.svg';
  import DatabaseGraphic from '$components/assets/images/product/global/DatabaseGraphic.svg';

  export let data: LayoutData;

  $: ({ supabase, session } = data)

  onMount(() => {
    const {
      data: { subscription },
    } = supabase.auth.onAuthStateChange((_event, session_) => {
      if (session_?.expires_at !== session?.expires_at) {
        invalidate('supabase:auth');
      }
    });

    return () => subscription.unsubscribe();
  })

  let isClosed = true;
  let hidden = true;
  let expanded = true;
  let handleDrawer = () => {
    hidden = !hidden;
    expanded = true;
  };
  let handleClick = () => {
    isClosed = !isClosed;
  };
  const hide = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    isClosed = true;
  };
  let isVerticalOpen = false;
  const handleDropdown = () => {
    hidden = !hidden;
  };

  let code = '"domain" {\n' + '\tfoo: string, \n' + '\tbar: string, \n' + '\tbaz: int\n' + '}';
</script>

<Navbar website={false}>
  <NavInner navDivClass="flex justify-between items-center">
    <NavToolbar navClass="px-3 mx-3 h-9 inline-flex space-x-4 items-center">
      <NavHamburger
        website={false}
        hamburgerClass="mx-2"
        on:click={() => handleDrawer()}
        on:click={() => (isClosed = true)}
        on:click={() => (isVerticalOpen = false)} />
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
    <NavToolbar navClass="px-3 h-9 inline-flex space-x-2.5 mx-3 items-center border-l border-gray-secondary dark:border-border-dark">
      {#if data.session?.user}
        <DropdownMenu product={true} name="profile" marginTop="mt-9" hidden>
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
            {data.session?.user.email}
          </DropdownItem>
          <DropdownItem href="/account">Account</DropdownItem>
          <DropdownDivider />
          <DropdownItem className="flex-1">
            <form method="post" action="/auth/sign-out">
              <button class="button block">Sign Out</button>
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
          <IconButton
            ><Icon
               name="support"
               size={18}
               iconColor="text-body-light dark:text-body-dark"
               pathName={Support}
               />
          </IconButton>
        </MaybeTooltip>
      {:else}
        <Button
          href="/auth/sign-in"
          gradient
          color="brandStackPrimary"
          size="sm"
          label="Sign In" />
        {/if}
      </NavToolbar>
  </NavInner>
</Navbar>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16"><ThemeSwitch /></span>

<Drawer placement="left" {hidden}>
  <Sidebar class={!isClosed ? 'w-[417px]' : 'w-[240px]'} {isClosed}>
    <SidebarWrapper>
      <Accordion class="flex flex-col" style="height: calc(100vh - 183px);">
        <SidebarContainer src={DesignLogo} title="Design" id="design" bind:expanded>
          <SidebarGroup>
            <SidebarDropdownWrapper
              label="	Schema"
              on:click={handleClick}
              bind:isVerticalOpen >
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
                    on:click={handleClick}
                    on:click={() => isVerticalOpen = false}>cancel</button>
                  <Button color="default" size="sm" label="Save" on:click class="" />
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
                   class="inline-flex mb-1"
                   pathName={Download}
                   /></Button
                       >
            </SidebarItem>
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DataLogo}
          href="/data"
          id="data"
          title="Data"
          on:click={() => (isClosed = true)}
          on:click={() => (isVerticalOpen = false)} >
          <SidebarGroup>
            <div class="h-full flex justify-center items-center">
              <div>
                <img src={DataGraphic} alt="Data Graphic" width="110" class="block mx-auto" />
                <Button
                  label="Keep me updated"
                  gradient
                  color="brandStackPrimary"
                  size="sm"
                  className="my-4"
                  on:click
                  class="block mr-auto ml-auto" />
              </div>
            </div>
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DomainFunctionsLogo}
          id="domain-functions"
          title="Domain Functions"
          on:click={() => (isClosed = true)}
          on:click={() => (isVerticalOpen = false)} >
          <SidebarGroup>
            <div class="h-full flex justify-center items-center">
              <div>
                <img
                  src={DomainFunctionsGraphic}
                  alt="Domain Functions Graphic"
                  width="150"
                  class="block mr-auto ml-auto"
                />
                <Button
                  label="Keep me updated"
                  gradient
                  color="brandStackPrimary"
                  size="sm"
                  className="my-4"
                  on:click
                  class="block mr-auto ml-auto"
                />
              </div>
            </div>
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DeployLogo}
          id="deploy"
          title="Deploy"
          on:click={() => (isClosed = true)}
          on:click={() => (isVerticalOpen = false)} >
          <SidebarGroup>
            <div class="h-full flex justify-center items-center">
              <div>
                <img
                  src={DeployGraphic}
                  alt="Deploy Graphic"
                  width="100"
                  class="block mr-auto ml-auto"
                />
                <Button
                  label="Keep me updated"
                  gradient
                  color="brandStackPrimary"
                  size="sm"
                  className="my-4"
                  on:click
                  class="block mr-auto ml-auto"
                />
              </div>
            </div>
          </SidebarGroup>
        </SidebarContainer>
        <SidebarContainer
          src={DatabaseLogo}
          id="database"
          title="Database"
          on:click={() => (isClosed = true)}
          on:click={() => (isVerticalOpen = false)} >
          <SidebarGroup>
            <div class="h-full flex justify-center items-center">
              <div>
                <img
                  src={DatabaseGraphic}
                  alt="Database Graphic"
                  width="130"
                  class="block mr-auto ml-auto"
                />
                <Button
                  label="Keep me updated"
                  gradient
                  color="brandStackPrimary"
                  size="sm"
                  className="my-4"
                  on:click
                  class="block mr-auto ml-auto"
                />
              </div>
            </div>
          </SidebarGroup>
        </SidebarContainer>
      </Accordion>
    </SidebarWrapper>
  </Sidebar>
</Drawer>

<main
  class="mt-16 left-0 relative transition-all duration-[200ms] ml-0"
  class:left-60={!hidden}
  class:ml-[177px]={!isClosed}
  class:ease-in={!isClosed} >
  <slot />
</main>
