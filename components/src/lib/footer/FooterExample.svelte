<script lang="ts">
	import classNames from 'classnames';
	import Footer from './Footer.svelte';
	import FooterBrand from './FooterBrand.svelte';
	import FooterLinkGroup from './FooterLinkGroup.svelte';
	import FooterLink from './FooterLink.svelte';
	import FooterLegal from './FooterLegal.svelte';
	import FooterCopyright from './FooterCopyright.svelte';
	import Container from '../Container.svelte';
	let containerClass: string | undefined = 'container';
	import Row from '../Row.svelte';
	let gridClass: string = 'grid lg:grid-cols-2 grid-col-1';
	import Column from '../Column.svelte';
	import Logo from '../assets/images/global/evidentDesignLogo.svg';
	export const alt = 'Brand Logo';
	export const href: string = '';
	export const product:string = '';
</script>

<div class="bg-white dark:bg-dark-1">
	<div class="bg-black dark:bg-white bg-opacity-[4%] dark:bg-opacity-[4%] py-[60px]">
		<Footer>
			<Container class={classNames('py-[60px]', containerClass)}>
				<Row class="grid md:grid-cols-4 grid-cols-3 gap-4 auto-cols-min">
					<Column class={classNames('md:block hidden py-4', gridClass)}>
						<FooterBrand src={Logo} />
					</Column>
					<Column class="flex-none col-span-3">
						<Row class="grid grid-cols-3 gap-4 py-4">
							<Column class="flex">
								<FooterLinkGroup label="Why Design?">
									<FooterLink label="Features" />
								</FooterLinkGroup>
							</Column>
							<Column class="flex">
								<FooterLinkGroup label="Learn">
									<FooterLink label="Blog" />
									<FooterLink label="Webinars" />
									<FooterLink label="Conferences" />
								</FooterLinkGroup>
							</Column>
							<Column class="flex">
								<FooterLinkGroup
									liClass="pb-2 w-full text-sm font-bold text-body-light dark:text-body-dark"
								>
									<FooterLink label="Docs" />
								</FooterLinkGroup>
							</Column>
						</Row>
						<Row class="grid grid-cols-3 gap-4 py-4">
							<Column class="flex">
								<FooterLinkGroup label="Service & Support">
									<FooterLink label="Support" />
								</FooterLinkGroup>
							</Column>
							<Column class="flex">
								<FooterLinkGroup label="Company">
									<FooterLink label="Team" />
									<FooterLink label="Contact" />
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
</div>
