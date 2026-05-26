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
  import SidebarItem from '$components/drawer/SidebarItem.svelte';
  import DrawerDetails from '$components/drawer/DrawerDetails.svelte';

  import { Accordion } from 'svelte-accessible-accordion';

  import TreeView from '$components/data/tree/TreeView.svelte';
  import TreeItem from '$components/data/tree/TreeItem.svelte';

  import Grid from '$components/design/Grid.svelte';

  import Button from '$components/Button.svelte';

  import Logo from '$components/assets/images/global/evidentStackLogo.svg';

  import DesignLogo from '$components/assets/images/product/design/evidentDesignLogo.svg';
  import DataLogo from '$components/assets/images/product/data/evidentDataLogo.svg';
  import DomainFunctionsLogo from '$components/assets/images/product/domainFunctions/evidentDomainFunctionsLogo.svg';
  import DeployLogo from '$components/assets/images/product/deploy/evidentDeployLogo.svg';
  import DatabaseLogo from '$components/assets/images/product/database/evidentDatabaseLogo.svg';

  import { mockGrid, mockDecider } from './mockGrid';

  let hidden = false;

  const toggleLeftNav = () => {
    hidden = !hidden;
  };

  let projectDescriptionOpen = false;
  const syncStatus = 0;

  let expandedLeftNavItem = 'design';

  $: designExpanded = expandedLeftNavItem === 'design';
  $: dataExpanded = expandedLeftNavItem === 'data';
  $: domainFunctionsExpanded = expandedLeftNavItem === 'domain-functions';
  $: deployExpanded = expandedLeftNavItem === 'deploy';
  $: dbExpanded = expandedLeftNavItem === 'db';

  const setExpandedLeftNavItem = (item: string) => {
    expandedLeftNavItem = item;
  };

  let isActive: string | number;

  const demoHref = '/';

  const tree_data = [
    {
      name: 'Autonomo Mobile iOS App',
      type: 'event-model',
      id: 1,
      children: [
        {
          name: 'Vehicle',
          type: 'read-model',
          id: 2,
          children: [
            { name: 'Add Vehicle', type: 'command', id: 3 },
            { name: 'Vehicle Added', type: 'event', id: 4 },
            { name: 'Vehicle Profile', type: 'read-model', id: 5 },
            { name: 'Owner App', type: 'interface', id: 6 }
          ]
        },
        {
          name: 'Ride',
          type: 'read-model',
          id: 7,
          children: [
            { name: 'Request Ride', type: 'command', id: 8 },
            { name: 'Ride Requested', type: 'event', id: 9 },
            { name: 'Ride Status', type: 'read-model', id: 10 },
            { name: 'Rider App', type: 'interface', id: 11 }
          ]
        }
      ]
    }
  ];
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
        <SidebarContainer
          src={DesignLogo}
          href={demoHref}
          id="design"
          title="Design"
          bind:expanded={designExpanded}
          on:click={() => setExpandedLeftNavItem('design')}
        >
          <SidebarGroup>
            <SidebarItem maxHeightNum={194} blank>
              <Button label="Export JSON" gradient color="ghost" size="sm" className="my-4" />
            </SidebarItem>
          </SidebarGroup>
        </SidebarContainer>

        <SidebarContainer
          src={DataLogo}
          href={demoHref}
          id="data"
          title="Data"
          bind:expanded={dataExpanded}
          on:click={() => setExpandedLeftNavItem('data')}
        >
          <SidebarGroup>
            <SidebarItem padding="p-0" maxHeightNum={322} blank>
              <TreeView {tree_data} let:item bind:isActive>
                {#if item.children}
                  <TreeItem
                    elementClass="flex items-center w-full group h-7"
                    href={`#tree-${item.id}`}
                  >
                    <span class="ml-1 text-xs font-semibold">{item.name}</span>
                  </TreeItem>
                {:else}
                  <TreeItem
                    elementClass="pl-[35px] flex items-center w-full h-7 text-body dark:text-body-dark text-default bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20]"
                    href={`#tree-${item.id}`}
                  >
                    <span class="ml-1">{item.name}</span>
                  </TreeItem>
                {/if}
              </TreeView>
            </SidebarItem>
          </SidebarGroup>
        </SidebarContainer>

        <SidebarContainer
          src={DomainFunctionsLogo}
          href={demoHref}
          id="domain-functions"
          title="Domain Functions"
          bind:expanded={domainFunctionsExpanded}
          on:click={() => setExpandedLeftNavItem('domain-functions')}
        >
          <SidebarGroup />
        </SidebarContainer>

        <SidebarContainer
          src={DeployLogo}
          href={demoHref}
          id="deploy"
          title="Deploy"
          bind:expanded={deployExpanded}
          on:click={() => setExpandedLeftNavItem('deploy')}
        >
          <SidebarGroup />
        </SidebarContainer>

        <SidebarContainer
          src={DatabaseLogo}
          href={demoHref}
          id="db"
          title="Database"
          bind:expanded={dbExpanded}
          on:click={() => setExpandedLeftNavItem('db')}
        >
          <SidebarGroup />
        </SidebarContainer>
      </Accordion>
    </Sidebar>
  </Drawer>

  <main
    class="relative left-0 right-0 transition-all duration-[200ms] pt-16 ml-0 h-screen overflow-hidden"
    class:ml-[240px]={!hidden}
  >
    <div
      class="absolute top-20 left-6 z-30 max-w-md rounded-md bg-white/90 dark:bg-dark-2/90 backdrop-blur-sm border border-gray-secondary dark:border-border-dark shadow-header px-4 py-3"
    >
      <p class="text-xs uppercase tracking-wide text-gray-brand-2 dark:text-gray-brand-4">
        Portfolio Demo
      </p>

      <h1 class="mt-1 text-sm font-bold text-body-light dark:text-body-dark">
        Evident Stack SaaS Interface
      </h1>

      <p class="mt-1 text-xs leading-normal text-body-light dark:text-gray-brand-4">
        A portfolio-safe demo of real product UI, design systems, event modeling, and front-end
        implementation work.
      </p>
    </div>
    <Grid mode="navigation" grid={mockGrid} decider={mockDecider} />
  </main>
</div>
