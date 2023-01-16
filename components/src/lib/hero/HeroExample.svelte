<script lang="ts">
	import classNames from 'classnames';
	import Hero from './Hero.svelte';
	import HeroContainer from './HeroContainer.svelte';
	import HeroCopy from './HeroCopy.svelte';
	import HeroFlexbox from './HeroFlexbox.svelte';
	import HeroGrid from './HeroGrid.svelte';
	import HeroImage from './HeroImage.svelte';
	import GradientDivider from '../utils/GradientDivider.svelte';
	import TextBrand from '../utils/TextBrand.svelte';
	import Button from '../Button.svelte';
	import Icon from '../Icon.svelte';
	import ArrowRight from '../icons/ArrowRight.svelte';
	import BackgroundImage from '../assets/images/homepage/hero/heroBgO.svg';
	import ForegroundImage from '../assets/images/homepage/hero/heroLaptop.svg';
	import ThemeSwitch from '../utils/ThemeSwitch.svelte';
	let heroFlexboxClass = 'flex justify-center';
	export let gradientDividerClass =
		'xl:mt-[-224px] lg:mt-[-150px] sm:mt-[-60px] mt-[-70px] h-[64px] bg-gradient-to-t from-gradient to-transparency opacity-20';
</script>

<span class="lg:block hidden top-0 right-0 z-20 absolute pt-3 pr-10"><ThemeSwitch /></span>
<Hero>
	<HeroContainer>
		<HeroGrid>
			<HeroFlexbox class={classNames('lg:mt-[56px] relative z-10 lg:px-0 px-4', heroFlexboxClass)}>
				<HeroCopy>
					<span slot="hero-header"
						>Design, implement, and operate event-driven information systems with <TextBrand
							>Evident Design</TextBrand
						></span
					>
					<span slot="hero-copy"
						>Give your customers a better cohesive customer experience. Your technical and
						non-technical teams can create a shared user journey together.</span
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
							size={12}><ArrowRight /></Icon
						>
					</Button>
				</HeroCopy>
			</HeroFlexbox>
			<HeroFlexbox
				class={classNames('flex-row lg:order-last order-first mx-auto', heroFlexboxClass)}
			>
				<HeroImage>
					<img
						src={ForegroundImage}
						alt="Laptop with oNote screenshot"
						width="100%"
						height="100%"
						class="absolute top-0 left-0"
					/>
					<img
						src={BackgroundImage}
						alt="Background"
						width="100%"
						height="100%"
						class="xl:mt-[-154px] lg:mt-[-111px] sm:mt-[-115px] mt-[-60px]"
					/>
				</HeroImage>
			</HeroFlexbox>
		</HeroGrid>
	</HeroContainer>
	<GradientDivider
		class={classNames(
			'xl:mt-[-224px] lg:mt-[-150px] sm:mt-[-60px] mt-[-70px]',
			gradientDividerClass
		)}
	/>
</Hero>
