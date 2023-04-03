<script lang="ts">
  import "../app.css";
  import type { LayoutData } from './$types'

  import Navbar from '$components/navbar/Navbar.svelte'
  import NavInner from '$components/navbar/NavInner.svelte'
  import NavToolbar from '$components/navbar/NavToolbar.svelte'
  import NavHamburger from '$components/navbar/NavHamburger.svelte'
  import NavBrand from '$components/navbar/NavBrand.svelte'
  import Button from '$components/Button.svelte';
  import IconButton from '$components/IconButton.svelte'
  import Icon from '$components/Icon.svelte'
  import Avatar from '$components/Avatar.svelte';
  import Home from '$components/icons/Home.svelte'
  import Docs from '$components/icons/Docs.svelte';
  import Support from '$components/icons/Support.svelte';
  import AdminPortal from '$components/icons/AdminPortal.svelte';
  import Logo from '$components/assets/images/global/evidentStackLogo.svg';

  export let data: LayoutData
</script>

<Navbar website={false}>
  <NavInner navDivClass='flex justify-between items-center'>
    <NavToolbar navClass='px-3 h-9 inline-flex space-x-4 mx-3 items-center'>
      <NavHamburger website={false} />
      <NavBrand src={Logo} height={28} logoClass='flex no-underline mx-3' />
      <div class="px-3 h-9 border-x border-gray-secondary flex items-center">
        <IconButton href='/' iconBtnClass="hover:bg-gray-brand-1/0">
          <Icon
            size={16}
            name='home'
            iconColor='text-body-light hover:text-brand-primary transition duration-200 ease-in'>
            <Home />
          </Icon>
        </IconButton>
      </div>
    </NavToolbar>
    <NavToolbar navClass='px-3 h-9 inline-flex space-x-2 mx-3 items-center border-l border-gray-secondary'>
      {#if data.session.user}
        <div class="tooltip tooltip-bottom" data-tip="Docs">
          <IconButton><Icon name="docs" size={18}><Docs /></Icon></IconButton>
        </div>
        <div class="tooltip tooltip-bottom" data-tip="Support">
          <IconButton><Icon name="support" size={18}><Support /></Icon></IconButton>
        </div>
        <div class="tooltip tooltip-bottom" data-tip="Admin Portal">
          <IconButton><Icon name="admin-portal" size={18}><AdminPortal /></Icon></IconButton>
        </div>
        <span class="flex justify-center px-3">
          <Avatar />
        </span>
      {:else}
        <Button href="/auth/sign-in" gradient color="brandStackPrimary" size="sm" on:click label="Sign In" />
      {/if}
    </NavToolbar>
  </NavInner>
</Navbar>

<main>
  <slot />
</main>
