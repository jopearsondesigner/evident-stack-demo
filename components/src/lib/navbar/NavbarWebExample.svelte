<script lang="ts">
	import classNames from 'classnames';
	import Navbar from './Navbar.svelte';
	import NavWrapper from './NavWrapper.svelte';
	import NabBrand from './NavBrand.svelte';
	import NavHamburger from './NavHamburger.svelte';
	import CloseButton from '$lib/utils/CloseButton.svelte';
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
	import Logo from '../assets/images/global/evidentStackLogo.svg';
	export let href: string = '';
	let ulClass: string = 'flex space-x-8 items-center';
	let website: boolean = true;
	export let hidden: boolean = false;
	let handleMenu = () => {
		hidden = !hidden;
	};
</script>

<Navbar>
	{#if !hidden}
		<NavWrapper {hidden}>
			<NabBrand src={Logo} {href} height={32} />
			<NavHamburger website={true} on:click={() => handleMenu()} />
			<Nav>
				<NavUl>
					<DropdownMenu name="Why Design?" website={true}>
						<NavUl>
							<NavLi>Features</NavLi>
						</NavUl>
					</DropdownMenu>
					<NavLi href="#">Docs</NavLi>
					<DropdownMenu name="Learn" website={true}>
						<NavUl>
							<NavLi href="#">Blog</NavLi>
							<NavLi href="#">Webinars</NavLi>
							<NavLi href="#">Conferences</NavLi>
						</NavUl>
					</DropdownMenu>
					<DropdownMenu name="Service & Support" website={true}>
						<NavUl>
							<NavLi href="#">Support</NavLi>
						</NavUl>
					</DropdownMenu>
					<DropdownMenu name="Company" website={true}>
						<NavUl>
							<NavLi href="#">Team</NavLi>
							<NavLi href="#">Contact</NavLi>
						</NavUl>
					</DropdownMenu>
				</NavUl>
				<NavUl class={classNames('lg:mr-12', ulClass)}>
					<NavLi href="#">Log In</NavLi>
					<NavLi>
						<Button
							href="https://app.onote.com/sign-up"
							gradient
							color="brandDesignPrimary"
							class="flex items-center"
							size="sm"
							label="Sign In"
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
			<MobileNav>
				<NavUl hidden>
					<NavLi hidden>
						<ThemeSwitch />
						<CloseButton on:click={() => handleMenu()} />
					</NavLi>
					<DropdownMenuMobile name="Why Design?">
						<NavUl hidden>
							<NavLi hidden>Features</NavLi>
						</NavUl>
					</DropdownMenuMobile>
					<NavLi hidden href="#">Docs</NavLi>
					<DropdownMenuMobile name="Learn">
						<NavUl hidden>
							<NavLi hidden>Blog</NavLi>
							<NavLi hidden>Webinars</NavLi>
							<NavLi hidden>Conferences</NavLi>
						</NavUl>
					</DropdownMenuMobile>
					<DropdownMenuMobile name="Service & Support">
						<NavUl hidden>
							<NavLi hidden>Support</NavLi>
						</NavUl>
					</DropdownMenuMobile>
					<DropdownMenuMobile name="Company">
						<NavUl hidden>
							<NavLi hidden>Team</NavLi>
							<NavLi hidden>Contact</NavLi>
						</NavUl>
					</DropdownMenuMobile>
				</NavUl>
				<NavUl mobileNavUl="p-3 border-t border-border-light dark:border-border-dark" hidden>
					<NavLi href="#" class="p-0" hidden footer>Log In</NavLi>
					<NavLi hidden footer>
						<Button
							href="https://app.onote.com/sign-up"
							gradient
							color="brandStackPrimary"
							class="flex items-center"
							size="sm"
							label="Sign In"
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
		</NavWrapper>
	{/if}
</Navbar>
