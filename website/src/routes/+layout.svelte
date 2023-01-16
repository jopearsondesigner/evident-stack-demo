<script lang="ts">
	import '../app.css';
	import classNames from 'classnames';
	import Navbar from '$lib/components/websitenavbar/Navbar.svelte';
	import NavWrapper from '$lib/components/websitenavbar/NavWrapper.svelte';
	import NabBrand from '$lib/components/websitenavbar/NavBrand.svelte';
	import NavHamburger from '$lib/components/websitenavbar/NavHamburger.svelte';
	import CloseButton from '$lib/components/utils/CloseButton.svelte';
	import ThemeSwitch from '$lib/components/utils/ThemeSwitch.svelte';
	import Nav from '$lib/components/websitenavbar/Nav.svelte';
	import MobileNav from '$lib/components/websitenavbar/MobileNav.svelte';
	import DropdownMenu from '$lib/components/dropdown/DropdownMenu.svelte';
	import DropdownMenuMobile from '$lib/components/dropdown/DropdownMenuMobile.svelte';
	import NavUl from '$lib/components/websitenavbar/NavUl.svelte';
	import NavLi from '$lib/components/websitenavbar/NavLi.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import ArrowRight from '$lib/components/icons/ArrowRight.svelte';
	import Footer from '$lib/components/footer/Footer.svelte';
	import FooterBrand from '$lib/components/footer/FooterBrand.svelte';
	import FooterLinkGroup from '$lib/components/footer/FooterLinkGroup.svelte';
	import FooterLink from '$lib/components/footer/FooterLink.svelte';
	import FooterLegal from '$lib/components/footer/FooterLegal.svelte';
	import FooterCopyright from '$lib/components/footer/FooterCopyright.svelte';
	import Container from '$lib/components/Container.svelte';
	let containerClass: string | undefined = 'container';
	import Row from '$lib/components//Row.svelte';
	let gridClass: string = 'grid lg:grid-cols-2 grid-col-1';
	import Column from '$lib/components/Column.svelte';
	import Logo from '$lib/components/assets/images/global/evidentStackLogo.svg';
	export const alt = 'Brand Logo';
	let ulClass: string = 'flex space-x-8 items-center';
	let website: boolean = false;
	export let hidden: boolean = false;
	let handleMenu = () => {
		hidden = !hidden;
	};
	import { clickOutside } from 'svelte-use-click-outside';
	export let bgColor: string = 'bg-gray-900';
	export let bgOpacity: string = 'bg-opacity-75';
	let backdropDivClass = classNames(
		'fixed top-0 left-0 z-20 w-full h-full',
		!hidden && bgColor,
		!hidden && bgOpacity
	);
</script>

<Navbar>
	{#if !hidden}
		<NavWrapper {hidden}>
			<NabBrand href="/" src={Logo} height={32} class="flex my-4" />
			<NavHamburger website on:click={() => handleMenu()}/>
			<Nav>
				<NavUl>
					<DropdownMenu name="Why Evident Stack?" {website}>
						<NavUl>
							<NavLi href="/features">Features</NavLi>
						</NavUl>
					</DropdownMenu>
					<NavLi href="http://docs.onote.com/">Docs</NavLi>
					<DropdownMenu name="Learn" {website}>
						<NavUl>
							<NavLi href="/blog">Blog</NavLi>
							<NavLi href="/webinars">Webinars</NavLi>
							<NavLi href="/conferences">Conferences</NavLi>
						</NavUl>
					</DropdownMenu>
					<DropdownMenu name="Service & Support" {website}>
						<NavUl>
							<NavLi href="https://support.onote.com/">Support</NavLi>
						</NavUl>
					</DropdownMenu>
					<DropdownMenu name="Company" {website}>
						<NavUl>
							<NavLi href="/team">Team</NavLi>
							<NavLi href="/contact">Contact</NavLi>
						</NavUl>
					</DropdownMenu>
				</NavUl>
				<NavUl class={classNames('lg:mr-12', ulClass)}>
					<NavLi href="https://app.onote.com/">Log In</NavLi>
					<NavLi>
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
								size={12}><ArrowRight /></Icon
							>
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
							<ThemeSwitch/>
							<CloseButton on:click={() => handleMenu()}/>
						</NavLi>
						<DropdownMenuMobile name="Why Design?">
							<NavUl hidden>
								<NavLi hidden href="/features">Features</NavLi>
							</NavUl>
						</DropdownMenuMobile>
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
						<NavLi href="https://app.onote.com/" class="p-0" hidden footer>Log In</NavLi>
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
									size={12}><ArrowRight /></Icon
								>
							</Button>
						</NavLi>
					</NavUl>
				</MobileNav>
			</div>
		</NavWrapper>
	{/if}
</Navbar>
<span class="lg:block hidden right-0 z-20 absolute pt-3 pr-10"><ThemeSwitch /></span>

{#if !hidden}
	<slot />
{:else}
	<div class={backdropDivClass} />
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
					<Row class="grid grid-cols-3 gap-4 py-4">
						<Column class="flex">
							<FooterLinkGroup label="Why Design?">
								<FooterLink label="Features" href="/features" />
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
							<FooterLinkGroup
								liClass="pb-2 w-full text-sm font-bold text-body-light dark:text-body-dark"
							>
								<FooterLink label="Docs" href="http://docs.onote.com/" />
							</FooterLinkGroup>
						</Column>
					</Row>
					<Row class="grid grid-cols-3 gap-4 py-4">
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
