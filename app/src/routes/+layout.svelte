<script lang="ts">
  import '../app.css';
  import type { LayoutData } from './$types';
  import { onMount } from 'svelte';
  import { invalidate } from '$app/navigation';
  import { handleSignOut } from '$lib/user';
  import { page } from '$app/stores';

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
  let expanded = false;
  let designExpanded = false;
  let dataExpanded = false;
  let domainfunctionsExpanded = false;
  let deployExpanded = false;
  let databaseExpanded = false;
  let path: string;

  $: path = $page.url.pathname;
  console.log(path);

  $: if (path == '/design/models') {
    designExpanded = true;
    console.log(path);
    console.log(dataExpanded);
  } else {
    designExpanded = false;
  }
  $: if (path == '/data') {
    dataExpanded = true;
    console.log(path);
    console.log(dataExpanded);
  } else {
    dataExpanded = false;
  }
  $: if (path == '/domain-functions') {
    domainfunctionsExpanded = true;
    console.log(path);
    console.log(dataExpanded);
  } else {
    domainfunctionsExpanded = false;
  }
  $: if (path == '/deploy') {
    deployExpanded = true;
    console.log(path);
    console.log(dataExpanded);
  } else {
    deployExpanded = false;
  }
  $: if (path == '/db') {
    databaseExpanded = true;
    console.log(path);
    console.log(dataExpanded);
  } else {
    databaseExpanded = false;
  }

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
  let hidden = false;
  let grid = true;
  $: handleDrawer = () => {
    hidden = !hidden;
    expanded = !expanded;
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

  let data: LayoutData;
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
    <NavToolbar
      navClass="px-3 h-9 inline-flex space-x-2.5 mx-3 items-center border-l border-gray-secondary dark:border-border-dark">
      <!-- {#if data.session.user} -->
      <DropdownMenu product={true} name="profile" marginTop="mt-9" hidden>
        <IconButton slot="button" margin="mx-2 mt-1">
          <Icon
            name="profile"
            size={32}
            viewBox="0 0 32 32"
            class="vertical-middle"
            iconColor=""
            pathName={Profile}
          />
        </IconButton>
        <DropdownItem padding="pt-2 pb-4 px-4" textOnly={true}>
          <!-- fake email for testing -->
          lutobor.kostalova66@centrum.cz</DropdownItem>
        <DropdownItem href="/account">Account</DropdownItem>
        <DropdownDivider />
        <DropdownItem className="flex-1" on:click={handleSignOut}>Sign Out</DropdownItem>
      </DropdownMenu>

      <MaybeTooltip tip="Docs" position="tooltip-bottom">
        <IconButton
          ><Icon
            name="docs"
            size={18}
            iconColor="text-body-light dark:text-body-dark"
            pathName={Docs}
          />
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
      <!-- {:else}
        <Button
          href="/auth/sign-in"
          gradient
          color="brandStackPrimary"
          size="sm"
          on:click
          label="Sign In"
        />
      {/if} -->
    </NavToolbar>
  </NavInner>
</Navbar>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16"><ThemeSwitch /></span>

<Drawer placement="left" {hidden}>
  <Sidebar class={!isClosed ? 'w-[417px]' : 'w-[240px]'} {isClosed}>
    <SidebarWrapper>
      <Accordion class="flex flex-col" style="height: calc(100vh - 183px);">
        <slot name="design" />
        <slot name="data" />
        <SidebarContainer
          src={DomainFunctionsLogo}
          id="domain-functions"
          title="Domain Functions"
          on:click={() => (isClosed = true)}
          on:click={() => (isVerticalOpen = false)}
          on:click={() => (grid = false)}
          bind:expanded={domainfunctionsExpanded} >
          <SidebarGroup />
        </SidebarContainer>
        <SidebarContainer
          src={DeployLogo}
          id="deploy"
          title="Deploy"
          on:click={() => (isClosed = true)}
          on:click={() => (isVerticalOpen = false)}
          on:click={() => (grid = false)}
          bind:expanded={deployExpanded} >
          <SidebarGroup />
        </SidebarContainer>
        <SidebarContainer
          src={DatabaseLogo}
          id="db"
          title="Database"
          on:click={() => (isClosed = true)}
          on:click={() => (isVerticalOpen = false)}
          on:click={() => (grid = false)}
          bind:expanded={databaseExpanded} >
          <SidebarGroup />
        </SidebarContainer>
      </Accordion>
    </SidebarWrapper>
  </Sidebar>
</Drawer>

<main
  class="{grid ? 'relative' : 'absolute'} left-0 right-0 transition-all duration-[200ms] ml-0"
  class:left-60={!hidden}
  class:ml-[177px]={!isClosed}
  class:ease-in={!isClosed} >
  <slot />
</main>
