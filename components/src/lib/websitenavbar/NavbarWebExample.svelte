<script lang='ts'>
    import classNames from 'classnames';
    import Navbar from './Navbar.svelte';
    import NavWrapper from './NavWrapper.svelte';
    import NabBrand from './NavBrand.svelte';
    import NavHamburger from './NavHamburger.svelte';
    import CloseButton from '../utils/CloseButton.svelte';
    import ThemeSwitch from '../utils/ThemeSwitch.svelte';
    import Nav from './Nav.svelte';
    import MobileNav from './MobileNav.svelte';
    import DropdownMenu from '../dropdown/DropdownMenu.svelte';
    import DropdownMenuMobile from '../dropdown/DropdownMenuMobile.svelte';
    import NavUl from './NavUl.svelte';
    import NavLi from './NavLi.svelte';
    import Button from '../Button.svelte';
    import Icon from '../Icon.svelte';
    import ArrowRight from '../icons/ArrowRight.svelte';
    export let href:string = '';
    let ulClass:string = 'flex space-x-8 items-center';
    let website:boolean = false;
    let acceptTerms = false;
    export let hidden:boolean = false;
    let handleMenu = () => {
		hidden = !hidden;
	};
</script>

<Navbar>
    {#if !hidden}
    <NavWrapper {hidden}>
        <NabBrand {href} height={24}/>
        <NavHamburger website on:click={() => handleMenu()}/>
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
            <NavUl class={classNames('lg:mr-12',ulClass)}>
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
                    <ThemeSwitch bind:checked={acceptTerms}/>
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

