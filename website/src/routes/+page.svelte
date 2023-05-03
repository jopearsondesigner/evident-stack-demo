<script lang="ts">
	import classNames from 'classnames';
	// import { LottiePlayer } from '@lottiefiles/svelte-lottie-player';
	import Player from '$components/utils/Player.svelte';

	import Carousel from 'svelte-carousel';
	import { browser } from '$app/environment';

	let carousel: { goToNext: () => void }; // for calling methods of the carousel instance

	const handleNextClick = () => {
		carousel.goToNext();
	};

	import Hero from '$components/hero/Hero.svelte';
	import HeroContainer from '$components/hero/HeroContainer.svelte';
	import HeroCopy from '$components/hero/HeroCopy.svelte';
	import HeroFlexbox from '$components/hero/HeroFlexbox.svelte';
	import HeroGrid from '$components/hero/HeroGrid.svelte';
	import HeroImage from '$components/hero/HeroImage.svelte';
	import TextBrand from '$components/utils/TextBrand.svelte';
	import Button from '$components/Button.svelte';
	import BackgroundImage from '$components/assets/images/homepage/hero/heroBgO.svg';
	import GettingStartedBottomLeft from '$components/assets/images/homepage/GettingStarted-BottomLeft.svg';
	import GettingStartedTopLeft from '$components/assets/images/homepage/GettingStarted-TopLeft.svg';
	import GettingStartedBottom from '$components/assets/images/homepage/GettingStarted-Bottom.svg';
	import GettingStartedCenter from '$components/assets/images/homepage/GettingStarted-Center.svg';
	import GettingStartedTopRight from '$components/assets/images/homepage/GettingStarted-TopRight.svg';
	import ForegroundImage from '$components/assets/images/homepage/hero/heroLaptop.svg';
	export let heroFlexboxClass = 'flex justify-center';
	export let gradientDividerClass =
		'h-[64px] bg-gradient-to-t from-gradient to-transparency opacity-20';
	import Container from '$components/Container.svelte';
	import SectionLabel from '$components/typography/SectionLabel.svelte';
	import Section from '$components/Section.svelte';
	import SectionHeader from '$components/typography/SectionHeader.svelte';
	import SectionCopy from '$components/typography/SectionCopy.svelte';
	import A from '$components/typography/A.svelte';
	import Row from '$components/Row.svelte';
	import Column from '$components/Column.svelte';
	import PoorSoftwareQuality from '$components/assets/images/homepage/poorSoftwareQuality.svg';
	import PoorSoftwareQualityDark from '$components/assets/images/homepage/poorSoftwareQualityDark.svg';
	let mainClass = 'bg-white dark:bg-dark-1 w-full h-auto z-10 relative pb-[128px]';
	export let gridClass: string = 'grid lg:grid-cols-2 grid-col-1 mt-[0px]';
	export let flexClass: string = 'flex justify-center items-center px-4';
	export let containerClass: string | undefined = 'container lg:pb-[128px] pb-16';
	import SeventyFive from '$components/assets/images/homepage/infoSeventyFive.svg';
	import SeventyEight from '$components/assets/images/homepage/infoSeventyEight.svg';
	import Eighty from '$components/assets/images/homepage/infoEighty.svg';
	import FifyFive from '$components/assets/images/homepage/infoFiftyFive.svg';
	import SeventyFiveDark from '$components/assets/images/homepage/infoSeventyFiveDark.svg';
	import SeventyEightDark from '$components/assets/images/homepage/infoSeventyEightDark.svg';
	import EightyDark from '$components/assets/images/homepage/infoEightyDark.svg';
	import FifyFiveDark from '$components/assets/images/homepage/infoFiftyFiveDark.svg';
	import GradientDivider from '$components/utils/GradientDivider.svelte';
	import FifteenOBg from '$components/assets/images/homepage/fifteen-o-bg.svg';
	import Heart from '$components/icons/Heart.svelte';
	import Docs from '$components/icons/Docs.svelte';
	import Icon from '$components/Icon.svelte';
	import ArrowRight from '$components/icons/ArrowRight.svelte';
	import ArrowLeft from '$components/icons/ArrowLeft.svelte';
	import Logo from '$components/assets/images/global/evidentDesignLogo.svg';
	import Navbar from '$components/navbar/Navbar.svelte';
	import NavWrapper from '$components/navbar/NavWrapper.svelte';
	import NabBrand from '$components/navbar/NavBrand.svelte';
	let website: boolean = true;
</script>

{#if browser}
	<Carousel
		bind:this={carousel}
		autoplay
		autoplayDuration={10000}
		let:showPrevPage
		let:showNextPage
	>
		<div
			slot="prev"
			on:click={showPrevPage}
			class="absolute h-full left-0 z-20 flex items-center justify-center custom-arrow custom-arrow-prev"
		>
			<Icon
				name="arrow-left"
				class="inline-flex ml-1 stroke-1"
				iconColor="fill-current text-gray-brand-4"
				size={40}
				pathName={ArrowLeft}
			/>
		</div>
		<div class="bg-white dark:bg-dark-1 z-10 grid">
			<Hero>
				<HeroContainer>
					<HeroGrid>
						<HeroFlexbox
							class={classNames(
								'lg:mt-[56px] md:mt-[136px] relative z-10 lg:px-0 px-4',
								heroFlexboxClass
							)}
						>
							<HeroCopy
								heroCopyClass="lg:mt-[11px] md:mt-[-140px] sm:mt-[-10px] mt-8 mt-[0px] lg:ml-16"
							>
								<span slot="hero-header"
									><TextBrand
										textBrand="text-transparent bg-clip-text bg-gradient-to-b from-stack-brand-700 via-stack-brand-500 to-stack-brand-300"
										>Evident Stack</TextBrand
									> helps software teams to design, implement, and operate event-driven apps and services</span
								>
								<span slot="hero-copy"
									>Each layer in the Stack can be used individually, but work best together since
									each layer builds upon the one before it</span
								>
							</HeroCopy>
						</HeroFlexbox>
						<HeroFlexbox
							class={classNames('flex-row lg:order-last order-first mx-auto', heroFlexboxClass)}
						>
							<div
								class="xl:max-w-[500px] lg:max-w-[400px] md:max-w-[400px] sm:max-w-[400px] max-w-[250px] w-auto flex justify-center items-center"
							>
								<Player src="lottie/teamCollaborationHero.json" />
							</div>
						</HeroFlexbox>
					</HeroGrid>
				</HeroContainer>
				<GradientDivider class={classNames('-mt-16', gradientDividerClass)} />
			</Hero>
		</div>
		<div>
			<Hero>
				<Navbar
					navbarWebClass="absolute bg-transparent shadow-none w-full max-h-12 px-3 py-1.5 text-body z-10"
					{website}
				>
					<NavWrapper>
						<NabBrand href="/" src={Logo} height={32} class="flex lg:my-4" logoClass="ml-8" />
					</NavWrapper>
				</Navbar>
				<HeroContainer>
					<HeroGrid>
						<HeroFlexbox
							class={classNames('lg:mt-[56px] relative z-10 lg:px-0 px-4', heroFlexboxClass)}
						>
							<HeroCopy
								heroCopyClass="lg:mt-[11px] md:-mt-8 sm:-mt-8 xl:pr-4 mt-8 sm:mt-[-130px] mt-[0px] mb-16 lg:ml-16"
							>
								<span slot="hero-header" class=""
									><TextBrand>Evident Design</TextBrand> helps teams to collaboratively design event-based
									information systems using Event Modeling</span
								>

								<Button
									slot="hero-cta"
									gradient
									color="brandDesignPrimary"
									size="sm"
									label="Sign In"
									class="flex items-center"
								>
									<Icon
										name="arrow-right"
										slot="icon"
										class="inline-flex ml-1 stroke-2"
										iconColor="fill-current text-white"
										size={12}
										pathName={ArrowRight}
									/>
								</Button>
							</HeroCopy>
						</HeroFlexbox>
						<HeroFlexbox
							class={classNames('flex-row lg:order-last order-first mx-auto', heroFlexboxClass)}
						>
							<HeroImage
								imgSize="xl:w-[897px] lg:w-[640px] md:w-[540px] sm:w-[540px] sm:mt-0 mt-[16px]"
							>
								<div
									class="h-full w-auto table overflow-auto md:-mt-[0] bg-no-repeat bg-center bg-cover"
									style="background-image: url({BackgroundImage})"
								>
									<img
										src={ForegroundImage}
										alt="Laptop with oNote screenshot"
										width="100%"
										height="100%"
										class="table-row"
									/>
								</div></HeroImage
							>
						</HeroFlexbox>
					</HeroGrid>
				</HeroContainer>
				<GradientDivider class={classNames('-mt-16', gradientDividerClass)} />
			</Hero>
		</div>
		<div
			slot="next"
			on:click={showNextPage}
			class="absolute h-full right-0 flex items-center justify-center custom-arrow custom-arrow-next"
		>
			<Icon
				name="arrow-right"
				class="inline-flex ml-1 stroke-1"
				iconColor="fill-current text-gray-brand-4"
				size={40}
				pathName={ArrowRight}
			/>
		</div>
	</Carousel>
{/if}

<main class={mainClass}>
	<!-- <button class="" on:click={handleNextClick}>Next</button> -->
	<Container class={containerClass}>
		<SectionLabel>The value of Evident Stack</SectionLabel>
		<Row class="grid grid-col-1 mt-[44px]">
			<Column class="flex mx-auto">
				<Player src="lottie/evidentStackVisualization-1280.json" />
			</Column>
		</Row>
	</Container>
	<div class="bg-black dark:bg-white bg-opacity-[4%] dark:bg-opacity-[4%]">
		<Container class={classNames('py-[60px]', containerClass)}>
			<Row class={classNames('lg:mt-[44px] mb-8 lg:py-[69px]', gridClass)}>
				<Column class="flex justify-center items-center lg:order-last mx-auto mt-8 lg:mt-0 px-4">
					<img
						src={PoorSoftwareQuality}
						class="w-[640px] block dark:hidden"
						width="auto"
						alt="Poor software quality"
					/>
					<img
						src={PoorSoftwareQualityDark}
						class="w-[640px] hidden dark:block"
						width="auto"
						alt="Poor software quality"
					/>
				</Column>
				<Column class={classNames('order-first lg:pr-12 lg:-mt-4 px-4')}>
					<Section name="benefit">
						<SectionHeader
							tag="h2"
							color="text-stack-brand-800 dark:text-stack-brand-300"
							class="mb-4 lg:pl-4 lg:text-left text-center"
							customSize="text-[1.5rem]"
						>
							Poor software design costs a lot
						</SectionHeader>
						<SectionCopy class="lg:pl-4 lg:text-left text-center mb-4">
							For 2020, the Consortium for Information & Software Quality (CISQ) determined the
							total Cost of Poor Software Quality (CPSQ) in the US is $2.08 trillion (T). The
							largest contributor to CPSQ is operational software failures. For 2020 estimated that
							it is ~$1.56 T, a 22% growth over 2 years – but that could be underestimated given the
							meteoric rise in cybersecurity failures, and that many failures go unreported. The
							underlying cause is primarily unmitigated flaws in the software.
						</SectionCopy>
						<SectionCopy class="lg:pl-4 lg:text-left text-center mb-4">
							The next largest contributor to CPSQ in unsuccessful development projects totaling
							$260 billion (B), which rose by 46% since 2018. The project failure rate has been
							steady at ~19% for over a decade. The underlying causes are varied, but one consistent
							theme has been the lack of attention to quality.
						</SectionCopy>
						<SectionCopy class="lg:pl-4 lg:text-left text-center mb-4">
							Source: <A href="https://www.it-cisq.org/cisq-files/pdf/CPSQ-2020-report.pdf"
								>The Cost of Poor Software Quality in the US: A 2020 Report</A
							>
						</SectionCopy>
					</Section>
				</Column>
			</Row>
			<Row
				class="grid lg:mt-8 mt-0 pt-0"
				customCols="xl:grid-cols-4 lg:grid-cols-2 md:grid-cols-2 sm:grid-cols-2"
			>
				<Column class="flex-col lg:order-last order-first mx-auto">
					<Section name="info" divClass="mb-8 flex items-center justify-center">
						<img
							src={SeventyFive}
							class="lg:w-[165px] sm:w-[133px] w-[110px] block dark:hidden"
							width="auto"
							alt="75%"
						/>
						<img
							src={SeventyFiveDark}
							class="lg:w-[165px] sm:w-[133px] w-[110px] hidden dark:block"
							width="auto"
							alt="75%"
						/>
					</Section>
					<Section name="info" divClass="xl:py-4 pt-0 pb-4">
						<SectionCopy class="text-center px-12" textSize="text-[1.125rem]">
							75% admit that their projects are typically destined to fail.
						</SectionCopy>
					</Section>
				</Column>
				<Column class="flex-col lg:order-last order-first mx-auto">
					<Section name="info" divClass="mb-8 flex items-center justify-center">
						<img
							src={SeventyEight}
							class="lg:w-[165px] sm:w-[133px] w-[110px] block dark:hidden"
							width="auto"
							alt="78%"
						/>
						<img
							src={SeventyEightDark}
							class="lg:w-[165px] sm:w-[133px] w-[110px] hidden dark:block"
							width="auto"
							alt="78%"
						/>
					</Section>
					<Section name="info" divClass="xl:py-4 pt-0 pb-4">
						<SectionCopy class="text-center px-12" textSize="text-[1.125rem]">
							78% feel their organization is typically out of sync with project requirements.
							Stakeholders should be more engaged in project requirements.
						</SectionCopy>
					</Section>
				</Column>
				<Column class="flex-col lg:order-last order-first mx-auto">
					<Section name="info" divClass="mb-8 flex items-center justify-center">
						<img
							src={Eighty}
							class="lg:w-[165px] sm:w-[133px] w-[110px] block dark:hidden"
							width="auto"
							alt="80%"
						/>
						<img
							src={EightyDark}
							class="lg:w-[165px] sm:w-[133px] w-[110px] hidden dark:block"
							width="auto"
							alt="80%"
						/>
					</Section>
					<Section name="info" divClass="xl:py-4 pt-0 pb-4">
						<SectionCopy class="text-center px-12" textSize="text-[1.125rem]">
							80% admit they spend at least half their time on rework.
						</SectionCopy>
					</Section>
				</Column>
				<Column class="flex-col lg:order-last order-first mx-auto">
					<Section name="info" divClass="mb-8 flex items-center justify-center">
						<img
							src={FifyFive}
							class="lg:w-[165px] sm:w-[133px] w-[110px] block dark:hidden"
							width="auto"
							alt="55%"
						/>
						<img
							src={FifyFiveDark}
							class="lg:w-[165px] sm:w-[133px] w-[110px] hidden dark:block"
							width="auto"
							alt="55%"
						/>
					</Section>
					<Section name="info" divClass="xl:py-4 pt-0 pb-4">
						<SectionCopy class="text-center px-12" textSize="text-[1.125rem]">
							Only 55% feel that the business objectives of their projects are clear to them.
						</SectionCopy>
					</Section>
				</Column>
			</Row>
		</Container>
		<GradientDivider class="=mt=16" />
	</div>
	<Container>
		<SectionLabel
			class="text-gray-primary dark:text-gray-brand-2 opacity-100 dark:opacity-60 font-bold lg:py-16 py-4 ml-4 text-left"
		>
			Getting Started
		</SectionLabel>
		<Row
			class="bg-white dark:bg-dark-1 lg:p-16 p-0 sm:p-8 mx-4 grid gap-4 lg:grid-cols-3 grid-col-1 bg-no-repeat lg:bg-[url('$components/assets/images/homepage/GettingStarted-BottomLeft.svg'),_url('$components/assets/images/homepage/GettingStarted-TopLeft.svg'),_url('$components/assets/images/homepage/GettingStarted-Bottom.svg'),_url('$components/assets/images/homepage/GettingStarted-Center.svg'),__url('$components/assets/images/homepage/GettingStarted-TopRight.svg')] lg:bg-[position:left_bottom,_left_-18px_top,_left_-180px_bottom_0px,_center_right_460px,_top_right] lg:bg-[length:140px_346px,_439px_221px,_602px_259px,_617px_225px,_160px_411px]"
		>
			<div class="lg:col-span-1">
				<Column class={classNames('h-full', flexClass)}>
					<Section name="benefit">
						<SectionHeader
							tag="h2"
							color="text-stack-brand-800 dark:text-stack-brand-300"
							class="mb-4 lg:pl-4 lg:text-left text-center"
							customSize="text-[1.875rem]"
						>
							Design right from the start
						</SectionHeader>
					</Section>
				</Column>
			</div>
			<div class="lg:col-span-1">
				<div
					class="p-4 sm:p-12 h-full bg-white bg-opacity-[72%] dark:bg-dark-1 dark:bg-opacity-[72%] rounded-md border border-border-light dark:border-border-dark"
				>
					<Icon
						name="heart"
						class="lg:m-4 mx-auto my-0"
						iconColor="text-red"
						size={28}
						pathName={Heart}
					/>
					<Section name="benefit" divClass="">
						<SectionHeader
							tag="h4"
							color="text-stack-brand-800 dark:text-stack-brand-300"
							classes="min-h-[60px] mb-4 lg:pl-4 lg:text-left text-center"
							customSize="text-[1.5rem]"
						>
							Join the Evident Design community
						</SectionHeader>
						<SectionCopy classes="min-h-[112px] lg:pl-4 lg:text-left text-center">
							Learn from others on their use cases and where they’ve levaraged Evident Design.
						</SectionCopy>
						<SectionCopy classes="lg:text-left text-center pl-4">
							<A href="/use-cases">
								Go to Use Cases<Icon
									name="arrow-right"
									size={12}
									iconColor="fill-current"
									class="stroke-2 ml-1"
									pathName={ArrowRight}
								/>
							</A>
						</SectionCopy>
					</Section>
				</div>
			</div>
			<div class="lg:col-span-1">
				<div
					class="p-4 sm:p-12 h-full bg-white bg-opacity-[72%] dark:bg-dark-1 dark:bg-opacity-[72%] rounded-md border border-border-light dark:border-border-dark"
				>
					<Icon
						name="docs"
						class="lg:m-4 mx-auto my-0"
						iconColor="fill-current text-body-light dark:text-white"
						size={28}
						pathName={Docs}
					/>
					<Section name="benefit" divClass="">
						<SectionHeader
							tag="h4"
							color="text-stack-brand-800 dark:text-stack-brand-300"
							classes="min-h-[60px] mb-4 lg:pl-4 lg:text-left text-center"
							customSize="text-[1.5rem]"
						>
							Read the docs
						</SectionHeader>
						<SectionCopy classes="min-h-[112px] lg:pl-4 lg:text-left text-center">
							Get a headstart on how to go in-depth with Evident Design.
						</SectionCopy>
						<SectionCopy classes="flex lg:text-left text-center pl-4">
							<A href="/docs">
								Go to Docs<Icon
									name="arrow-right"
									size={12}
									iconColor="fill-current"
									class="stroke-2 ml-1"
									pathName={ArrowRight}
								/>
							</A>
						</SectionCopy>
					</Section>
				</div>
			</div>
		</Row>
	</Container>
</main>
