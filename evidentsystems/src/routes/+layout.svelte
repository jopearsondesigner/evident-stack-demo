<script lang="ts">
  import '../app.css';
  import classNames from 'classnames';
  import { goto } from '$app/navigation';
  export let href: string;
  import Navbar from '$components/navbar/Navbar.svelte';
  import NavWrapper from '$components/navbar/NavWrapper.svelte';
  import NabBrand from '$components/navbar/NavBrand.svelte';
  import NavHamburger from '$components/navbar/NavHamburger.svelte';
  import CloseButton from '$components/utils/CloseButton.svelte';
  import ThemeSwitch from '$components/utils/ThemeSwitch.svelte';
  import Nav from '$components/navbar/Nav.svelte';
  import MobileNav from '$components/navbar/MobileNav.svelte';
  import DropdownMenu from '$components/dropdown/DropdownMenu.svelte';
  import DropdownMenuMobile from '$components/dropdown/DropdownMenuMobile.svelte';
  import NavUl from '$components/navbar/NavUl.svelte';
  import NavLi from '$components/navbar/NavLi.svelte';
  import Button from '$components/Button.svelte';
  import Icon from '$components/Icon.svelte';
  import ArrowRight from '$components/icons/ArrowRight.svelte';
  import Footer from '$components/footer/Footer.svelte';
  import FooterBrand from '$components/footer/FooterBrand.svelte';
  import FooterLinkGroup from '$components/footer/FooterLinkGroup.svelte';
  import FooterLink from '$components/footer/FooterLink.svelte';
  import FooterLegal from '$components/footer/FooterLegal.svelte';
  import FooterCopyright from '$components/footer/FooterCopyright.svelte';
  import Container from '$components/Container.svelte';
  let containerClass: string | undefined = 'container';
  import Row from '$components//Row.svelte';
  let gridClass: string = 'grid lg:grid-cols-2 grid-col-1';
  import Column from '$components/Column.svelte';
  import Logo from '$components/assets/images/evidentsystems/evidentSystemsLogo-White.svg';
  export const alt = 'Brand Logo';
  let ulClass: string = 'flex space-x-8 items-center';
  export let hidden: boolean = false;
  let handleMenu = () => {
    hidden = !hidden;
  };
  import { clickOutside } from 'svelte-use-click-outside';
  export let bgColor: string = 'bg-gray-900';
  export let bgOpacity: string = 'bg-opacity-75';
  let backdropDivClass = classNames(
    'fixed top-0 left-0 z-30 w-full h-full',
    !hidden && bgColor,
    !hidden && bgOpacity
  );
</script>

<Navbar
  website={true}
  navbarWebClass="relative shadow-header bg-dark-2 w-full px-3 text-gray-brand-1 z-50"
>
  {#if !hidden}
    <NavWrapper {hidden}>
      <NabBrand href="/" src={Logo} height={24} class="flex my-4" />
      <NavHamburger website on:click={() => handleMenu()} />
      <Nav navClass="hidden lg:flex-1 lg:flex lg:items-center lg:justify-end">
        <NavUl class={classNames('lg:mr-12', ulClass)}>
          <NavLi>
            <Button color="evidentsystems" class="flex" size="sm" href="#contact" label="Contact" />
          </NavLi>
        </NavUl>
      </Nav>
    </NavWrapper>
  {:else}
    <NavWrapper
      hidden
      mobileNavDiv="lg:hidden m-2 absolute top-0 inset-x-0 rounded-lg shadow-lg bg-dark-2 border border-border-dark"
    >
      <div use:clickOutside={handleMenu}>
        <MobileNav>
          <NavUl hidden>
            <NavLi hidden>
              <CloseButton on:click={() => handleMenu()} />
            </NavLi>
          </NavUl>
          <NavUl mobileNavUl="p-3 border-t border-border-dark" hidden>
            <NavLi hidden footer>
              <Button
                color="evidentsystems"
                class="flex mt-2 "
                size="sm"
                label="Contact"
                href="#contact"
                on:click={() => handleMenu()}
                on:click={() => goto(href)}
              />
            </NavLi>
          </NavUl>
        </MobileNav>
      </div>
    </NavWrapper>
  {/if}
</Navbar>

{#if !hidden}
  <slot />
{:else}
  <div class={classNames(backdropDivClass)} />
  <slot />
{/if}

<Footer class="bg-dark-2 py-[60px]">
  <Container class={classNames('py-[60px]', containerClass)}>
    <Row class="grid md:grid-cols-4 grid-cols-3 gap-4 auto-cols-min">
      <Column class={classNames('md:block hidden py-4', gridClass)}>
        <FooterBrand src={Logo} height={24} />
      </Column>
    </Row>
    <section class="mt-16 text-center">
      <FooterLegal
        aClass="text-xs text-body-dark hover:text-body-light transition duration-200 ease-in mx-1"
      />
      <FooterCopyright
        by="Evident Systems LLC"
        spanClass="text-center text-xs text-body-dark mt-4"
      />
    </section>
  </Container>
</Footer>
