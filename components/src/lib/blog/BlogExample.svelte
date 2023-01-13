<script lang="ts">
	import Navbar from '../websitenavbar/Navbar.svelte';
	import NavWrapper from '../websitenavbar/NavWrapper.svelte';
	import Nav from '../websitenavbar/Nav.svelte';
	import NavUl from '../websitenavbar/NavUl.svelte';
	import NavLi from '../websitenavbar/NavLi.svelte';
	import Container from '../Container.svelte';
	import Section from '../Section.svelte';
	import SectionHeader from '../typography/SectionHeader.svelte';
	import SectionCopy from '../typography/SectionCopy.svelte';
	import SectionLabel from '../typography/SectionLabel.svelte';
	import Row from '../Row.svelte';
	import A from '../typography/A.svelte';
	import Column from '../Column.svelte';
	import Button from '../Button.svelte';
	import Icon from '../Icon.svelte';
	import ArrowRight from '../icons/ArrowRight.svelte';
	import Article from './Article.svelte';
	import FeaturedBadge from './FeaturedBadge.svelte';
	import BlogCta from './BlogCta.svelte';
	import FeaturedBlogImage from '../assets/images/blog/blog-photo-4.png';
	import BlogImage1 from '../assets/images/blog/blog-photo-1.png';
</script>

<Navbar navbarClass="w-full bg-black bg-opacity-90 px-4 py-3 sticky top-0 z-10 shadow-md">
	<NavWrapper navDivClass="sm:container">
		<Nav website navWebClass="block">
			<NavUl class="flex justify-center items-center pb-0 mr-4 space-x-8 list-none">
				<NavLi class="text-body-light hover:text-body-light">Blog</NavLi>
				<NavLi href="#" class="text-body-dark hover:text-body-light">Webinars</NavLi>
				<NavLi href="#" class="text-body-dark hover:text-body-light">Conferences</NavLi>
			</NavUl>
		</Nav>
	</NavWrapper>
</Navbar>
<Container>
	<SectionLabel>oNote blog</SectionLabel>
	<Section name="blog" divClass="md:pt-16 pt-0 pb-[136px]">
		<Row customCols="lg:grid-cols-6">
			<Column
				class="lg:block hidden order-first block m-4 pl-4 ml-0"
				customSpan="lg:col-span-4 col-span-2"
			>
				<FeaturedBadge />
				<A href="#"
					><img
						src={FeaturedBlogImage}
						alt="New Features Announced at Kafka Summit 2021"
						class="object-cover w-full h-full"
						width="100%"
						height="100%"
					/></A
				>
			</Column>
			<Column
				class="order-first m-4 lg:pl-4 lg:pl-4 pl-4 ml-0"
				customSpan="lg:col-span-2 col-span-2"
			>
				<Article>
					<FeaturedBadge divClass="lg:hidden" />
					<div class="lg:hidden">
						<A href="#"
							><img
								src={FeaturedBlogImage}
								alt="New Features Announced at Kafka Summit 2021"
								class="object-cover"
							/></A
						>
					</div>
					<p
						class="text-sm text-black text-opacity-90 dark:text-gray-primary uppercase font-bold lg:pt-0 mt-2 mb-4"
					>
						Blog
					</p>
					<Section name="blog">
						<SectionHeader tag="h2" customSize="text-[30px]">
							<A
								class="text-gray-brand-2 hover:text-gray-brand-1 leading-[1.27] transition duration-200 ease-in"
								href="#"
							>
								New Features Announced at Kafka Summit 2021
							</A>
						</SectionHeader>
						<SectionCopy>
							<div class="line-clamp-4">
								The oNote team just got back from presenting at Kafka Summit Americas 2021, where we
								announced lots of exciting new features that put event modeling at the heart of
								event-driven system design and implementation! Let’s review all of these enhanced
								and new features now available in oNote.
							</div>
							<A href="#">Read More</A>
						</SectionCopy>
					</Section>
					<Section
						name="blog"
						sectionClass="h-full flex flex-col justify-end items-start"
						divClass="w-full"
					>
						<div class="text-sm text-gray-brand-4 mt-4">2021-10-14</div>
						<BlogCta label="Join the community.">
							<Button
								gradient
								color="brandStackPrimary"
								size="sm"
								label="Get Started"
								class="flex items-center"
								href="#"
							>
								<Icon
									name="arrow-right"
									slot="icon"
									class="inline-flex ml-1 stroke-2"
									iconColor="fill-current text-white"
									size={12}><ArrowRight /></Icon
								>
							</Button>
						</BlogCta>
					</Section>
				</Article>
			</Column>
			<Column
				class="m-4 border-b border-border-light dark:border-border-dark"
				customSpan="col-span-2"
			>
				<Article customMargin="mt-0">
					<A href="#"
						><img src={BlogImage1} alt="Event Modeling Explained" class="object-cover" /></A
					>
					<p
						class="text-sm text-black text-opacity-90 dark:text-gray-primary uppercase font-bold lg:pt-0 mt-2 mb-4"
					>
						Blog
					</p>
					<Section name="blog" divClass="h-full">
						<SectionHeader tag="h2" customSize="text-[30px]">
							<A
								class="text-gray-brand-2 hover:text-gray-brand-1 leading-[1.27] transition duration-200 ease-in"
								href="#"
							>
								Event Modeling Explained
							</A>
						</SectionHeader>
						<SectionCopy>
							<div class="line-clamp-2">
								It’s no secret that tech developers and senior executives don’t understand each
								other. Execs tend to have lofty ideas for the perfect customer experience.
								Developers are constrained to what the technology is actually capable of. Luckily,
								product managers came along to try and mediate the developer-executive relationship.
								But even still, miscommunication and frustration seem to dominate software
								development these days.
							</div>
							<A href="#">Read More</A>
						</SectionCopy>
						<div class="text-sm text-gray-brand-4 mt-4">2020-08-24</div>
					</Section>
				</Article>
			</Column>
		</Row>
	</Section>
</Container>
