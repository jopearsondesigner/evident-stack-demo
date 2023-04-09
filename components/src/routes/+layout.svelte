<script lang="ts">
  import 'components/src/app.css';
  import { sineIn } from 'svelte/easing';
  import Grid from '$lib/design/Grid.svelte';
  import Drawer from '$lib/drawer/Drawer.svelte';
  import Sidebar from '$lib/drawer/Sidebar.svelte';
  import SidebarWrapper from '$lib/drawer/SidebarWrapper.svelte';
  import SidebarGroup from '$lib/drawer/SidebarGroup.svelte';
  import SidebarDropdownWrapper from '$lib/drawer/SidebarDropdownWrapper.svelte';
  import SidebarDropdownItem from '$lib/drawer/SidebarDropdownItem.svelte';
  import SidebarItem from '$lib/drawer/SidebarItem.svelte';
  import Design from '$lib/icons/Design.svelte';
  import Implement from '$lib/icons/Implement.svelte';
  import Collaborations from '$lib/icons/Collaborations.svelte';
  import ManageSchemas from '$lib/icons/ManageSchemas.svelte';
  import ExportJson from '$lib/icons/Json.svelte';
  import Schema from '$lib/icons/Schema.svelte';
  import EventModels from '$lib/icons/EventModels.svelte';
  import AddIcon from '$lib/icons/AddIcon.svelte';
  import ThemeSwitch from '$lib/utils/ThemeSwitch.svelte';
  import Navbar from '$lib/navbar/Navbar.svelte';
  import NavHamburger from '$lib/navbar/NavHamburger.svelte';
  import NavBrand from '$lib/navbar/NavBrand.svelte';
  import NavInner from '$lib/navbar/NavInner.svelte';
  import NavToolbar from '$lib/navbar/NavToolbar.svelte';
  import Icon from '$lib/Icon.svelte';
  import Button from '$lib/Button.svelte';
  import IconButton from '$lib/IconButton.svelte';
  import Avatar from '$lib/Avatar.svelte';
  import Docs from '$lib/icons/Docs.svelte';
  import Support from '$lib/icons/Support.svelte';
  import Logo from '$lib/assets/images/global/evidentDesignLogo.svg';
  import A from '$lib/typography/A.svelte';
  import MaybeTooltip from '$lib/utils/MaybeTooltip.svelte';
  let headerItem = true;
  export let alt = 'Brand Logo';
  export let user = true;
  let hidden = true;
  let handleDrawer = () => {
    hidden = !hidden;
  };
  let website: boolean = false;
</script>

<Navbar>
  <NavInner navDivClass="flex justify-between items-center">
    <NavToolbar navClass="px-3 h-9 inline-flex space-x-4 mx-3 items-center">
      <NavHamburger {website} on:click={() => handleDrawer()} />
      <NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3" />
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
            /></IconButton
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

<span class="lg:block hidden right-0 z-20 absolute pt-4 pr-10"><ThemeSwitch /></span>
<Drawer {hidden}>
  <Sidebar>
    <SidebarWrapper slot="eventModel">
      <SidebarGroup>
        <SidebarItem label="Event Models" {headerItem}>
          <Icon
            slot="leftIcon"
            name="event-models"
            size={16}
            iconColor="fill-current text-gray-brand-4 dark:text-gray-brand-2"
            pathName={EventModels}
          />
          <Icon
            slot="rightIcon"
            name="add-icon"
            size={20}
            iconColor="fill-current text-body dark:text-white"
            pathName={AddIcon}
          />
        </SidebarItem>
        <SidebarItem label="This is an Event Model!" active />
        <SidebarDropdownWrapper label="Collaborations">
          <Icon
            slot="icon"
            name="collaborations"
            size={16}
            iconColor="fill-current text-gray-brand-4 dark:text-gray-brand-2"
            pathName={Collaborations}
          />
          <SidebarDropdownItem label="This is a Collaboration!" />
        </SidebarDropdownWrapper>
      </SidebarGroup>
      <SidebarGroup>
        <SidebarDropdownWrapper horizontal label="Design">
          <Icon
            slot="icon"
            name="design"
            iconColor="fill-current text-gray-brand-4 dark:text-brand-gray-2"
            size={15}
            pathName={Design}
          />
          <SidebarDropdownItem label="Start a Collaboration">
            <Icon
              slot="icon"
              name="collaborations"
              iconColor="fill-current text-gray-brand-4 dark:text-brand-gray-2"
              size={16}
              pathName={Collaborations}
            />
          </SidebarDropdownItem>
          <SidebarDropdownItem label="Manage Schemas">
            <Icon
              slot="icon"
              name="manage-schemas"
              iconColor="fill-current text-gray-brand-4 dark:text-brand-gray-2"
              size={16}
              pathName={ManageSchemas}
            />
          </SidebarDropdownItem>
        </SidebarDropdownWrapper>
        <SidebarDropdownWrapper horizontal label="Implement">
          <Icon
            slot="icon"
            name="implement"
            iconColor="fill-current text-gray-brand-4 dark:text-brand-gray-2"
            size={16}
            pathName={Implement}
          />
          <SidebarDropdownItem label="Export JSON">
            <Icon
              slot="icon"
              name="export-jason"
              iconColor="fill-current text-gray-brand-4 dark:text-brand-gray-2"
              size={16}
              pathName={ExportJson}
            />
          </SidebarDropdownItem>
          <SidebarDropdownItem label="Generate Avro">
            <Icon
              slot="icon"
              name="generate-avro"
              iconColor="fill-current text-gray-brand-4 dark:text-brand-gray-2"
              size={16}
              pathName={Schema}
            />
          </SidebarDropdownItem>
        </SidebarDropdownWrapper>
      </SidebarGroup>
    </SidebarWrapper>
  </Sidebar>
</Drawer>

<main class="left-0 h-screen relative transition-all duration-[200ms]" class:left-60={!hidden}>
  <slot />
</main>
