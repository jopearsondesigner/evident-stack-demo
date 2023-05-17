<script lang="ts">
  import '../../app.css';
  import type { LayoutData } from './$types';
  import { onMount } from 'svelte';
  import { invalidate } from '$app/navigation';

  import Navbar from '$components/navbar/Navbar.svelte';
  import NavInner from '$components/navbar/NavInner.svelte';
  import NavToolbar from '$components/navbar/NavToolbar.svelte';
  import NavBrand from '$components/navbar/NavBrand.svelte';
  import IconButton from '$components/IconButton.svelte';
  import MaybeTooltip from '$components/utils/MaybeTooltip.svelte';
  import Icon from '$components/Icon.svelte';
  import Home from '$components/icons/Home.svelte';
  import Docs from '$components/icons/Docs.svelte';
  import Support from '$components/icons/Support.svelte';
  import Profile from '$components/icons/Profile.svelte';
  import Logo from '$components/assets/images/global/evidentStackLogo.svg';

  import ThemeSwitch from '$components/utils/ThemeSwitch.svelte';

  import DropdownMenu from '$components/dropdown/DropdownMenu.svelte';
  import DropdownItem from '$components/dropdown/DropdownItem.svelte';
  import DropdownDivider from '$components/dropdown/DropdownDivider.svelte';
  import { enhance } from '$app/forms';
  import Button from '$components/Button.svelte';

  // === Authentication

  export let data: LayoutData;

  $: ({ session } = data)

  let signOutLoading = false;

  function handleSignOutSubmit() {
    signOutLoading = true;
    return async () => {
      signOutLoading = false;
    };
  }

  // === End Authentication
</script>

<Navbar website={false}>
  <NavInner navDivClass="flex justify-between items-center">
    <NavToolbar navClass="px-3 mx-3 h-9 inline-flex space-x-4 items-center">
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
      {#if session}
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

<main class="relative left-0 right-0 transition-all duration-[200ms] ml-0 pt-16">
  <slot />
</main>
