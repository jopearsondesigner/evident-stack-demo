<script lang="ts">
  import Navbar from '$components/navbar/Navbar.svelte';
  import NavInner from '$components/navbar/NavInner.svelte';
  import NavToolbar from '$components/navbar/NavToolbar.svelte';
  import NavHamburger from '$components/navbar/NavHamburger.svelte';
  import NavBrand from '$components/navbar/NavBrand.svelte';
  import Logo from '$components/assets/images/global/evidentStackLogo.svg';
  import Drawer from '$components/drawer/Drawer.svelte';
  import Sidebar from '$components/drawer/Sidebar.svelte';
  import SidebarContainer from '$components/drawer/SidebarContainer.svelte';
  import SidebarGroup from '$components/drawer/SidebarGroup.svelte';
  import DesignLogo from '$components/assets/images/product/design/evidentDesignLogo.svg';
  import DataLogo from '$components/assets/images/product/data/evidentDataLogo.svg';
  import DomainFunctionsLogo from '$components/assets/images/product/domainFunctions/evidentDomainFunctionsLogo.svg';

  let hidden = false;

  const toggleLeftNav = () => {
    hidden = !hidden;
  };
</script>

<Navbar website={false}>
  <NavInner navDivClass="flex justify-between items-center">
    <NavToolbar navClass="px-3 mx-3 h-9 inline-flex space-x-4 items-center">
      <NavHamburger website={false} hamburgerClass="mx-2" on:click={toggleLeftNav} />
      <NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3 cursor-default" />
    </NavToolbar>
  </NavInner>
</Navbar>

<Drawer placement="left" bind:hidden>
  <Sidebar class="w-[240px]">
    <div class="px-4 py-4 border-b border-gray-secondary dark:border-border-dark">
      <h2 class="text-sm font-semibold text-body-light dark:text-body-dark">
        Autonomo Mobile iOS App
      </h2>
      <p class="mt-1 text-xs text-gray-primary dark:text-gray-primary">
        Portfolio-safe demo project
      </p>
    </div>

    <SidebarContainer src={DesignLogo} href="#" id="design" title="Design" expanded={true}>
      <SidebarGroup />
    </SidebarContainer>

    <SidebarContainer src={DataLogo} href="#" id="data" title="Data" expanded={true}>
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
  </Sidebar>
</Drawer>

<main class="relative min-h-screen pt-16 ml-[240px] bg-gray-50 dark:bg-dark">
  <div class="flex h-[calc(100vh-64px)]">
    <!-- Center Workspace -->
    <section class="flex-1 border-r border-gray-secondary dark:border-border-dark">
      <div class="h-full flex items-center justify-center">
        <div class="text-center">
          <h1 class="text-2xl font-semibold text-body-light dark:text-body-dark">
            Event Modeling Workspace
          </h1>

          <p class="mt-2 text-sm text-gray-primary">Portfolio-safe SaaS interaction demo</p>
        </div>
      </div>
    </section>

    <!-- Right Inspector Panel -->
    <aside
      class="w-[320px] bg-white dark:bg-dark-2 border-l border-gray-secondary dark:border-border-dark"
    >
      <div class="px-5 py-4 border-b border-gray-secondary dark:border-border-dark">
        <h2 class="text-sm font-semibold text-body-light dark:text-body-dark">Placement Details</h2>
      </div>

      <div class="p-5 space-y-4">
        <div>
          <label class="block text-xs uppercase tracking-wide text-gray-primary">
            Placement Name
          </label>

          <div class="mt-1 px-3 py-2 rounded border border-gray-secondary dark:border-border-dark">
            Vehicle Added
          </div>
        </div>

        <div>
          <label class="block text-xs uppercase tracking-wide text-gray-primary"> Type </label>

          <div class="mt-1 px-3 py-2 rounded border border-gray-secondary dark:border-border-dark">
            Domain Event
          </div>
        </div>
      </div>
    </aside>
  </div>
</main>
