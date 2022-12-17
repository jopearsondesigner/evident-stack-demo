<script lang='ts'>
    import classNames from 'classnames';
    import Navbar from './Navbar.svelte';
    import NavWrapper from './NavWrapper.svelte';
    import NabBrand from './NavBrand.svelte';
    import NavHamburger from './NavHamburger.svelte';
    import CloseButton from '../utils/CloseButton.svelte';
    import ModeSwitch from '../utils/ModeSwitch.svelte';
    import Nav from './Nav.svelte';
    import MobileNav from './MobileNav.svelte';
    import DropdownMenu from '../dropdown/DropdownMenu.svelte';
    import DropdownMenuMobile from '../dropdown/DropdownMenuMobile.svelte';
    import NavUl from './NavUl.svelte';
    import NavLi from './NavLi.svelte';
	import Logo from '../assets/images/global/onoteLogo.svg';
    import Button from '../Button.svelte';
    import Icon from '../Icon.svelte';
    import ArrowRight from '../icons/ArrowRight.svelte';
    import ArrowDown from '../icons/ArrowDown.svelte';
    import Close from '../icons/Close.svelte';
    import Menu from '../icons/Menu.svelte';
	export let alt="Brand Logo";
    export let href: string;
    export let navbarClass='header shadow-md z-30 relative bg-white dark:bg-dark-2';
    export let navDivClass='xl:container mx-auto flex justify-between items-center lg:py-0 lg:justify-start lg:space-x-8';
    export let navBrandClass='flex min-w-[103.26px] no-underline';
    export let logoClass='lg:h-[28px] h-[25px] ml-4';
    export let product='oNote';
    export let navHamburgerClass='mr-4 lg:hidden bg-none rounded-md p-2 inline-flex items-center justify-center text-body-1 dark:text-body-4 no-underline';
    export let navClass='hidden lg:flex-1 lg:flex lg:items-center lg:justify-between';
    export let navUlClass='flex space-x-8 items-center';
    export let navLiClass='text-sm whitespace-nowrap text-gray-brand-1 hover:text-gray-brand-1 dark:text-gray-brand-4 dark:hover:text-gray-brand-1 transition duration-300 ease-in-out inline-flex no-underline';
    let website = false;
    let acceptTerms = false;
    export let hidden:boolean = false;
    let handleMenu = () => {
		hidden = !hidden;
	};
</script>

<Navbar>
    {#if !hidden}
    <NavWrapper {hidden}>
        <NabBrand {href}/>
        <NavHamburger on:click={() => handleMenu()}/>
        <Nav>
            <NavUl>
                <DropdownMenu name='Why oNote?' {website}>
                    <NavUl>
                        <NavLi>Features</NavLi>
                    </NavUl>
                </DropdownMenu>
                <NavLi href="#">Docs</NavLi>
                <DropdownMenu name='Learn' {website}>
                    <NavUl>
                        <NavLi href="#">Blog</NavLi>
                        <NavLi href="#">Webinars</NavLi>
                        <NavLi href="#">Conferences</NavLi>
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
            <NavUl class={classNames('lg:mr-12',navUlClass)}>
                <NavLi href="#">Log In</NavLi>
                <NavLi>
                    <Button href='https://app.onote.com/sign-up' gradient color='brandSecondary' class='flex items-center' size='sm' label='Sign In'>
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
                    <ModeSwitch bind:checked={acceptTerms}/>
                    <CloseButton on:click={() => handleMenu()}/>
                </NavLi>
                <DropdownMenuMobile name='Why oNote?'>
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
                    <Button href='https://app.onote.com/sign-up' gradient color='brandSecondary' class='flex items-center' size='sm' label='Sign In'>
                        <Icon name='arrow-right' slot=icon class='stroke-2 ml-1' iconColor='fill-current' size={12}><ArrowRight/></Icon>
                    </Button>
                </NavLi>
            </NavUl>
        </MobileNav>
    </NavWrapper>
    {/if}
</Navbar>

