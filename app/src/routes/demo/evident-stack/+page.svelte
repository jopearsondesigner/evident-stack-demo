<script lang="ts">
  import ThemeSwitch from '$components/utils/ThemeSwitch.svelte';
  import Navbar from '$components/navbar/Navbar.svelte';
  import NavInner from '$components/navbar/NavInner.svelte';
  import NavToolbar from '$components/navbar/NavToolbar.svelte';
  import NavHamburger from '$components/navbar/NavHamburger.svelte';
  import NavBrand from '$components/navbar/NavBrand.svelte';

  import Drawer from '$components/drawer/Drawer.svelte';
  import Sidebar from '$components/drawer/Sidebar.svelte';
  import SidebarContainer from '$components/drawer/SidebarContainer.svelte';
  import SidebarGroup from '$components/drawer/SidebarGroup.svelte';
  import DrawerDetails from '$components/drawer/DrawerDetails.svelte';
  import { Accordion } from 'svelte-accessible-accordion';

  import Grid from '$components/design/Grid.svelte';

  import Logo from '$components/assets/images/global/evidentStackLogo.svg';
  import DesignLogo from '$components/assets/images/product/design/evidentDesignLogo.svg';
  import DataLogo from '$components/assets/images/product/data/evidentDataLogo.svg';
  import DomainFunctionsLogo from '$components/assets/images/product/domainFunctions/evidentDomainFunctionsLogo.svg';
  import DeployLogo from '$components/assets/images/product/deploy/evidentDeployLogo.svg';
  import DatabaseLogo from '$components/assets/images/product/database/evidentDatabaseLogo.svg';

  import { mockGrid, mockDecider } from './mockGrid';

  let hidden = true;

  const toggleLeftNav = () => {
    hidden = !hidden;
  };

  let projectDescriptionOpen = false;
  const syncStatus = 0;
</script>

<svelte:head>
  <title>Evident Stack Demo | Portfolio</title>
</svelte:head>

<div class="min-h-screen">
  <Navbar website={false}>
    <NavInner navDivClass="flex justify-between items-center">
      <NavToolbar navClass="px-3 mx-3 h-9 inline-flex space-x-4 items-center">
        <NavHamburger website={false} hamburgerClass="mx-2" on:click={toggleLeftNav} />
        <NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3 cursor-default" />
      </NavToolbar>
    </NavInner>
  </Navbar>

  <span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16">
    <ThemeSwitch />
  </span>

  <Drawer placement="left" bind:hidden>
    <Sidebar class="w-[240px]">
      <DrawerDetails
        name="Autonomo Mobile iOS App"
        description="Portfolio-safe demo of the Evident Stack event modeling interface."
        sync_status={syncStatus}
        bind:isOpen={projectDescriptionOpen}
      />

      <Accordion>
        <SidebarContainer src={DesignLogo} href="#" id="design" title="Design" expanded={true}>
          <SidebarGroup />
        </SidebarContainer>

        <SidebarContainer src={DataLogo} href="#" id="data" title="Data" expanded={false}>
          <SidebarGroup />
        </SidebarContainer>

        <SidebarContainer
          src={DomainFunctionsLogo}
          href="#"
          id="domain-functions"
          title="Domain Functions"
          expanded={false}
        >
          <SidebarGroup />
        </SidebarContainer>

        <SidebarContainer src={DeployLogo} href="#" id="deploy" title="Deploy" expanded={false}>
          <SidebarGroup />
        </SidebarContainer>

        <SidebarContainer src={DatabaseLogo} href="#" id="db" title="Database" expanded={false}>
          <SidebarGroup />
        </SidebarContainer>
      </Accordion>
    </Sidebar>
  </Drawer>

  <main
    class="relative left-0 right-0 transition-all duration-[200ms] pt-16 ml-0 h-screen overflow-hidden"
    class:ml-[240px]={!hidden}
  >
    <Grid mode="navigation" grid={mockGrid} decider={mockDecider} />
  </main>
</div>
