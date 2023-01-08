<script lang='ts'>
    import '../app.css';
    import classNames from 'classnames';
    import Navbar from 'components/websitenavbar/Navbar.svelte';
    import NavWrapper from 'components/websitenavbar/NavWrapper.svelte';
    import NabBrand from 'components/websitenavbar/NavBrand.svelte';
    import NavHamburger from 'components/websitenavbar/NavHamburger.svelte';
    import CloseButton from 'components/utils/CloseButton.svelte';
    // import ThemeSwitch from 'components/utils/ThemeSwitch.svelte';
    import Nav from 'components/websitenavbar/Nav.svelte';
    import MobileNav from 'components/websitenavbar/MobileNav.svelte';
    import DropdownMenu from 'components/dropdown/DropdownMenu.svelte';
    import DropdownMenuMobile from 'components/dropdown/DropdownMenuMobile.svelte';
    import NavUl from 'components/websitenavbar/NavUl.svelte';
    import NavLi from 'components/websitenavbar/NavLi.svelte';
    import Button from 'components/Button.svelte';
    import Icon from 'components/Icon.svelte';
    import ArrowRight from 'components/icons/ArrowRight.svelte';
    import Footer from 'components/footer/Footer.svelte';
    import FooterBrand from 'components/footer/FooterBrand.svelte';
    import FooterLinkGroup from 'components/footer/FooterLinkGroup.svelte';
    import FooterLink from 'components/footer/FooterLink.svelte';
    import FooterLegal from 'components/footer/FooterLegal.svelte';
    import FooterCopyright from 'components/footer/FooterCopyright.svelte';
    import Container from 'components/Container.svelte';
	let containerClass: string | undefined='container';
    import Row from 'components//Row.svelte';
	let gridClass: string='grid lg:grid-cols-2 grid-col-1';
    import Column from 'components/Column.svelte';
	import Logo from 'components/assets/images/global/evidentStackLogo.svg';
	export let alt="Brand Logo";
    export let navBrandClass='flex min-w-[103.26px] no-underline';
    export { navBrandClass as class };
    export let product='oNote';
    export let href:string = '';
    let ulClass:string = 'flex space-x-8 items-center';
    let website:boolean = false;
    let checked = false;
    export let hidden:boolean = false;
    let handleMenu = () => {
		hidden = !hidden;
	};
</script>

<Navbar>
    {#if !hidden}
    <NavWrapper {hidden}>
        <NabBrand href='/' src={Logo} height={32} class='flex my-4'/>
        <NavHamburger website on:click={() => handleMenu()}/>
        <Nav>
            <NavUl>
                <DropdownMenu name='Why Evident Stack?' {website}>
                    <NavUl>
                        <NavLi href='/features'>Features</NavLi>
                    </NavUl>
                </DropdownMenu>
                <NavLi href="#">Docs</NavLi>
                <DropdownMenu name='Learn' {website}>
                    <NavUl>
                        <NavLi href='/blog'>Blog</NavLi>
                        <NavLi href='/webinars'>Webinars</NavLi>
                        <NavLi href='/conferences'>Conferences</NavLi>
                    </NavUl>
                </DropdownMenu>
                <DropdownMenu name='Service & Support' {website}>
                    <NavUl>
                        <NavLi href="#">Support</NavLi>
                    </NavUl>
                </DropdownMenu>
                <DropdownMenu name='Company' {website}>
                    <NavUl>
                        <NavLi href="#">Team</NavLi>
                        <NavLi href="#">Contact</NavLi>
                    </NavUl>
                </DropdownMenu>
            </NavUl>
            <NavUl class={classNames('lg:mr-12',ulClass)}>
                <NavLi href="#">Log In</NavLi>
                <NavLi>
                    <Button href='https://app.onote.com/sign-up' gradient color='brandStackPrimary' class='flex items-center' size='sm' label='Sign In'>
                        <Icon name='arrow-right' slot=icon class='stroke-2 ml-1' iconColor='fill-current' size={12}><ArrowRight/></Icon>
                    </Button>
                </NavLi>
            </NavUl>
        </Nav>
    </NavWrapper>
    {:else}
    <NavWrapper hidden>
        <MobileNav>
            <NavUl hidden>
                <NavLi hidden>
                    <!-- <ThemeSwitch {checked}/> -->
                    <CloseButton on:click={() => handleMenu()}/>
                </NavLi>
                <DropdownMenuMobile name='Why Design?'>
                    <NavUl hidden>
                        <NavLi hidden>
                            Features
                        </NavLi>
                    </NavUl>
                </DropdownMenuMobile>
                <NavLi hidden href="#">
                    Docs
                </NavLi>
                <DropdownMenuMobile name='Learn'>
                    <NavUl hidden>
                        <NavLi hidden>
                            Blog
                        </NavLi>
                        <NavLi hidden>
                            Webinars
                        </NavLi>
                        <NavLi hidden>
                            Conferences
                        </NavLi>
                    </NavUl>
                </DropdownMenuMobile>
                <DropdownMenuMobile name='Service & Support'>
                    <NavUl hidden>
                        <NavLi hidden>
                            Support
                        </NavLi>
                    </NavUl>
                </DropdownMenuMobile>
                <DropdownMenuMobile name='Company'>
                    <NavUl hidden>
                        <NavLi hidden>
                            Team
                        </NavLi>
                        <NavLi hidden>
                            Contact
                        </NavLi>
                    </NavUl>
                </DropdownMenuMobile>
            </NavUl>
            <NavUl mobileNavUl='p-3 border-t border-border-light dark:border-border-dark' hidden>
                <NavLi href="#" class="p-0" hidden footer>Log In</NavLi>
                <NavLi hidden footer>
                    <Button href='https://app.onote.com/sign-up' gradient color='brandStackPrimary' class='flex items-center' size='sm' label='Sign In'>
                        <Icon name='arrow-right' slot=icon class='stroke-2 ml-1' iconColor='fill-current' size={12}><ArrowRight/></Icon>
                    </Button>
                </NavLi>
            </NavUl>
        </MobileNav>
    </NavWrapper>
    {/if}
</Navbar>

<slot/>

<div class="bg-white dark:bg-dark-1">
    <Footer class='bg-black dark:bg-white bg-opacity-[4%] dark:bg-opacity-[4%] py-[60px]'>
        <Container class={classNames('py-[60px]',containerClass)}>
            <Row class='grid md:grid-cols-4 grid-cols-3 gap-4 auto-cols-min'>
                <Column class={classNames('md:block hidden py-4', gridClass)}>
                    <FooterBrand src={Logo}/>
                </Column>
                <Column class='flex-none col-span-3'>
                    <Row class='grid grid-cols-3 gap-4 py-4'>
                        <Column class='flex'>
                            <FooterLinkGroup label='Why Design?'>
                                <FooterLink label='Features'/>
                            </FooterLinkGroup>
                        </Column>
                        <Column class='flex'>
                            <FooterLinkGroup label='Learn'>
                                <FooterLink label='Blog'/>
                                <FooterLink label='Webinars'/>
                                <FooterLink label='Conferences'/>
                            </FooterLinkGroup>
                        </Column>
                        <Column class='flex'>
                            <FooterLinkGroup liClass='pb-2 w-full text-sm font-bold text-body-light dark:text-body-dark'>
                                <FooterLink label='Docs'/>
                            </FooterLinkGroup>
                        </Column>
                    </Row>
                    <Row class='grid grid-cols-3 gap-4 py-4'>
                        <Column class='flex'>
                            <FooterLinkGroup label='Service & Support'>
                                <FooterLink label='Support'/>
                            </FooterLinkGroup>
                        </Column>
                        <Column class='flex'>
                            <FooterLinkGroup label='Company'>
                                <FooterLink label='Team'/>
                                <FooterLink label='Contact'/>
                            </FooterLinkGroup>
                        </Column>
                    </Row>
                </Column>
            </Row>
            <section class="mt-16 text-center">
                <FooterLegal/>
                <FooterCopyright by='Evident Systems LLC'/>
            </section>
        </Container>
    </Footer>
</div>
