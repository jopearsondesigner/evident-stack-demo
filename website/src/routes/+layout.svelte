<script lang="ts">
  import '../app.css';
  import classNames from 'classnames';
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
  import Modal from '$components/Modal.svelte';
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
  import Logo from '$components/assets/images/global/evidentStackLogo.svg';
  export const alt = 'Brand Logo';
  let ulClass: string = 'flex space-x-8 items-center';
  let website: boolean = true;
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

  import Form from '$components/form/Form.svelte';
  import Input from '$components/form/Input.svelte';
  import Textarea from '$components/form/Textarea.svelte';
  import A from '$components/typography/A.svelte';
  import Submit from '$components/form/Submit.svelte';
  import EarlyAccessSignupForm from '../lib/EarlyAccessSignupForm.svelte';

  $: Modal_id = 'modal-editing-window'; // TODO: derive this from model ID?

  let contactModal = false;
  export let open = false;
  const hide = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    open = false;
  };

  const handleContactModel = async () => {
    console.log('TODO: submit form');
  };
</script>

<Navbar website={true}>
  {#if !hidden}
    <NavWrapper {hidden}>
      <NabBrand href="/" src={Logo} height={32} class="flex my-4" />
      <NavHamburger website on:click={() => handleMenu()} />
      <Nav>
        <NavUl>
          <NavLi href="http://docs.onote.com/">Docs</NavLi>
          <DropdownMenu name="Learn" marginTop="mt-12" {website}>
            <NavUl>
              <NavLi href="/blog">Blog</NavLi>
              <NavLi href="/webinars">Webinars</NavLi>
              <NavLi href="/conferences">Conferences</NavLi>
            </NavUl>
          </DropdownMenu>
          <DropdownMenu name="Service & Support" marginTop="mt-12" {website}>
            <NavUl>
              <NavLi href="https://support.onote.com/">Support</NavLi>
            </NavUl>
          </DropdownMenu>
          <DropdownMenu name="Company" marginTop="mt-12" {website}>
            <NavUl>
              <NavLi href="/team">Team</NavLi>
              <NavLi href="/contact">Contact</NavLi>
            </NavUl>
          </DropdownMenu>
        </NavUl>
        <NavUl class={classNames('lg:mr-12', ulClass)}>
          <!-- <NavLi href="https://app.onote.com/">Log In</NavLi> -->
          <NavLi>
            <Button
              gradient
              color="brandStackPrimary"
              class="flex items-center"
              size="sm"
              on:click={() => (contactModal = true)}
              label="Sign Up for Early Access"
            >
              <Icon
                name="arrow-right"
                slot="icon"
                class="stroke-2 ml-1"
                iconColor="fill-current"
                size={12}
                pathName={ArrowRight}
              />
            </Button>
          </NavLi>
        </NavUl>
      </Nav>
    </NavWrapper>
  {:else}
    <NavWrapper hidden>
      <div use:clickOutside={handleMenu}>
        <MobileNav>
          <NavUl hidden>
            <NavLi hidden>
              <ThemeSwitch />
              <CloseButton on:click={() => handleMenu()} />
            </NavLi>
            <NavLi hidden href="http://docs.onote.com/">Docs</NavLi>
            <DropdownMenuMobile name="Learn">
              <NavUl hidden>
                <NavLi hidden href="/blog">Blog</NavLi>
                <NavLi hidden href="/webinars">Webinars</NavLi>
                <NavLi hidden href="/conferences">Conferences</NavLi>
              </NavUl>
            </DropdownMenuMobile>
            <DropdownMenuMobile name="Service & Support">
              <NavUl hidden>
                <NavLi hidden href="https://support.onote.com/">Support</NavLi>
              </NavUl>
            </DropdownMenuMobile>
            <DropdownMenuMobile name="Company">
              <NavUl hidden>
                <NavLi hidden href="/team">Team</NavLi>
                <NavLi hidden href="/contact">Contact</NavLi>
              </NavUl>
            </DropdownMenuMobile>
          </NavUl>
          <NavUl mobileNavUl="p-3 border-t border-border-light dark:border-border-dark" hidden>
            <!-- <NavLi href="https://app.onote.com/" class="p-0" hidden footer>Log In</NavLi> -->
            <NavLi hidden footer>
              <Button
                gradient
                color="brandStackPrimary"
                class="flex items-center"
                size="sm"
                on:click={() => (contactModal = true)}
                label="Sign Up for Early Access"
              >
                <Icon
                  name="arrow-right"
                  slot="icon"
                  class="stroke-2 ml-1"
                  iconColor="fill-current"
                  size={12}
                  pathName={ArrowRight}
                />
              </Button>
            </NavLi>
          </NavUl>
        </MobileNav>
      </div>
    </NavWrapper>
  {/if}
</Navbar>
<span class="lg:block hidden right-0 z-20 absolute pt-3 pr-10 mt-16"><ThemeSwitch /></span>

{#if !hidden}
  <slot />
{:else}
  <div class={classNames(backdropDivClass)} />
  <slot />
{/if}

<div class={classNames('bg-white dark:bg-dark-1')}>
  <Footer class="bg-black dark:bg-white bg-opacity-[4%] dark:bg-opacity-[4%] py-[60px]">
    <Container class={classNames('py-[60px]', containerClass)}>
      <Row class="grid md:grid-cols-4 grid-cols-3 gap-4 auto-cols-min">
        <Column class={classNames('md:block hidden py-4', gridClass)}>
          <FooterBrand src={Logo} />
        </Column>
        <Column class="flex-none col-span-3">
          <Row class="grid grid-cols-3 md:grid-cols-4 gap-4 py-4">
            <Column class="flex">
              <FooterLinkGroup
                liClass="pb-2 w-full text-sm font-bold text-body-light dark:text-body-dark"
              >
                <FooterLink label="Docs" href="http://docs.onote.com/" />
              </FooterLinkGroup>
            </Column>
            <Column class="flex">
              <FooterLinkGroup label="Learn">
                <FooterLink label="Blog" href="/blog" />
                <FooterLink label="Webinars" href="/webinars" />
                <FooterLink label="Conferences" href="/conferences" />
              </FooterLinkGroup>
            </Column>
            <Column class="flex">
              <FooterLinkGroup label="Service & Support">
                <FooterLink label="Support" href="https://support.onote.com/" />
              </FooterLinkGroup>
            </Column>
            <Column class="flex">
              <FooterLinkGroup label="Company">
                <FooterLink label="Team" href="/team" />
                <FooterLink label="Contact" href="/contact" />
              </FooterLinkGroup>
            </Column>
          </Row>
        </Column>
      </Row>
      <section class="mt-16 text-center">
        <FooterLegal />
        <FooterCopyright by="Evident Systems LLC" />
      </section>
    </Container>
  </Footer>
</div>

<Modal bind:open={contactModal} size="xs" autoclose title="Contact Form" color="brand">
  <div class="text-center w-full inline-flex justify-center items-center p px-6">
    <EarlyAccessSignupForm />
  </div>
  <!-- <div slot="footer" class="mx-3 flex items-end space-x-3" /> -->
</Modal>
