<script lang="ts">
  import Navbar from '$components/navbar/Navbar.svelte';
  import NavInner from '$components/navbar/NavInner.svelte';
  import NavToolbar from '$components/navbar/NavToolbar.svelte';
  import NavHamburger from '$components/navbar/NavHamburger.svelte';
  import NavBrand from '$components/navbar/NavBrand.svelte';
  import Logo from '$components/assets/images/global/evidentStackLogo.svg';
</script>

<Navbar website={false}>
  <NavInner navDivClass="flex justify-between items-center">
    <NavToolbar navClass="px-3 mx-3 h-9 inline-flex space-x-4 items-center">
      <NavHamburger website={false} hamburgerClass="mx-2" />
      <NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3 cursor-default" />
    </NavToolbar>
  </NavInner>
</Navbar>

<main class="pt-16 p-8">
  <h1>Evident Stack Demo</h1>
  <p>Portfolio-safe SaaS demo shell.</p>
</main>
