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
</script>

<Navbar website={false}>
  <NavInner navDivClass="flex justify-between items-center">
    <NavToolbar navClass="px-3 mx-3 h-9 inline-flex space-x-4 items-center">
      <NavHamburger website={false} hamburgerClass="mx-2" />
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

<main class="pt-16 p-8">
  <h1>Evident Stack Demo</h1>
  <p>Portfolio-safe SaaS demo shell.</p>
</main>
