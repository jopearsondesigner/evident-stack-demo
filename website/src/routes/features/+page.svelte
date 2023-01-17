<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import SectionLabel from '$lib/components/typography/SectionLabel.svelte';
	import Row from '$lib/components/Row.svelte';
	import Column from '$lib/components/Column.svelte';
	import Section from '$lib/components/Section.svelte';
	import SectionHeader from '$lib/components/typography/SectionHeader.svelte';
	import SectionCopy from '$lib/components/typography/SectionCopy.svelte';
	import TabWrapper from '$lib/components/tabs/TabWrapper.svelte';
	import TabHead from '$lib/components/tabs/TabHead.svelte';
	import TabHeadItem from '$lib/components/tabs/TabHeadItem.svelte';
	import TabContentItem from '$lib/components/tabs/TabContentItem.svelte';
	let mainClass: string = 'w-full h-auto z-10 relative';
	let containerClass = 'sm:container mx-auto lg:py-16 py-4';
	import FeaturesDesign from '$lib/components/assets/images/features/featuresDesign.svg';
	import FeaturesImplement from '$lib/components/assets/images/features/featuresImplement.svg';
	import FeaturesOperate from '$lib/components/assets/images/features/featuresOperate.svg';
	import FeaturesLogo from '$lib/components/assets/images/features/featuresLogo.svg';
	let activeTabValue = 1;
	const handleClick = (tabValue: number) => () => {
		activeTabValue = tabValue;
	};
</script>

<main class={mainClass}>
	<Container class="sm:container mx-auto pb-4">
		<SectionLabel>Evident Stack features</SectionLabel>
	</Container>
	<Container class={containerClass}>
		<Row class="grid lg:grid-cols-6">
			<Column class="hidden lg:block px-4" customSpan="lg:col-span-3">
				<TabHead ulClass="gap-4 sm:flex overflow-hidden -mb-px">
					<TabHeadItem
						id={1}
						on:click={handleClick(1)}
						{activeTabValue}
						liClass="list-none border-[3px] border-gray-brand-4 rounded-lg shadow-lg w-full mb-4 sm:mb-0 max-w-[318px] mx-auto"
						buttonClass="featureButtons h-full border-2 border-border-light rounded-[5px] shadow-md px-4 py-2 w-full transition ease-in-out duration-300"
						hClass="font-header font-bold text-[30px] text-body-light dark:text-gray-brand-4 whitespace-nowrap my-2"
						name="Design"
					>
						<img src={FeaturesDesign} alt="Design" width="auto" class="mx-auto hidden sm:block" />
						<div class=" min-h-[40px]">
							<p class="text-body-light dark:text-body-dark text-sm pb-0">Build models</p>
						</div>
					</TabHeadItem>
					<TabHeadItem
						id={2}
						on:click={handleClick(2)}
						{activeTabValue}
						liClass="list-none border-[3px] border-gray-brand-4 rounded-lg shadow-lg w-full mb-4 sm:mb-0 max-w-[318px] mx-auto"
						buttonClass="featureButtons h-full border-2 border-border-light rounded-[5px] shadow-md px-4 py-2 w-full transition ease-in-out duration-300"
						hClass="font-header font-bold text-[30px] text-body-light dark:text-gray-brand-4 whitespace-nowrap my-2"
						name="Implement"
					>
						<img
							src={FeaturesImplement}
							alt="Implement"
							width="auto"
							class="mx-auto hidden sm:block"
						/>
						<div class=" min-h-[40px]">
							<p class="text-body-light dark:text-body-dark text-sm pb-0">
								Export event models and generate Avro
							</p>
						</div>
					</TabHeadItem>
					<TabHeadItem
						id={3}
						on:click={handleClick(3)}
						{activeTabValue}
						liClass="list-none border-[3px] border-gray-brand-4 rounded-lg shadow-lg w-full mb-4 sm:mb-0 max-w-[318px] mx-auto"
						buttonClass="featureButtons h-full border-2 border-border-light rounded-[5px] shadow-md px-4 py-2 w-full transition ease-in-out duration-300"
						hClass="font-header font-bold text-[30px] text-body-light dark:text-gray-brand-4 whitespace-nowrap my-2"
						name="Operate"
					>
						<img src={FeaturesOperate} alt="Operate" width="auto" class="mx-auto hidden sm:block" />
						<div class=" min-h-[40px]">
							<p class="text-body-light dark:text-body-dark text-sm pb-0">Visualize data</p>
						</div>
					</TabHeadItem>
				</TabHead>
				<div class="hidden lg:block">
					<img src={FeaturesLogo} alt="Evident Stack" width="auto" class="mx-auto max-w-[418px]" />
				</div>
			</Column>
			<Column
				class="mx-4 lg:col-span-3 px-4 mt-8 lg:mt-0 box-border border-t lg:border-l border-border-light dark:border-border-dark"
			>
				<TabWrapper>
					<TabHead ulClass="flex list-none pb-0 mt-[-30px] lg:ml-4">
						<TabHeadItem id={1} on:click={handleClick(1)} {activeTabValue} name="Design" />
						<TabHeadItem id={2} on:click={handleClick(2)} {activeTabValue} name="Implemant" />
						<TabHeadItem id={3} on:click={handleClick(3)} {activeTabValue} name="Operate" />
					</TabHead>
					<TabContentItem id={1} {activeTabValue}>
						<div class="flex justify-center items-center px-4">
							<Section name="feature" sectionClass="lg:mt-0">
								<SectionCopy
									pClass="text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold"
									textSize="text-sm"
								>
									Design
								</SectionCopy>
								<SectionHeader
									tag="h2"
									hClass="leading-[1.27] font-header font-bold mb-4 text-left"
									color="text-gray-brand-2 dark:text-gray-brand-4"
									customSize="lg:text-[30px] text-[24px]"
								>
									Build models
								</SectionHeader>
								<SectionCopy
									pClass="mb-4 leading-6 text-left text-body-light dark:text-body-dark"
									textSize="text-[18px]"
								>
									Build valid event-driven models that design a blueprint for all personas.
								</SectionCopy>
								<SectionCopy
									pClass="leading-6 text-left text-body-light dark:text-body-dark"
									textSize="text-[18px]"
								>
									Define datatype schemas at the model and model object levels. Reference parent
									schemas for convenient reuse.
								</SectionCopy>
							</Section>
						</div>
					</TabContentItem>
					<TabContentItem id={2} {activeTabValue}>
						<div class="flex justify-center items-center px-4">
							<Section name="feature" sectionClass="lg:mt-0">
								<SectionCopy
									pClass="text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold"
									textSize="text-sm"
								>
									Implement
								</SectionCopy>
								<SectionHeader
									tag="h2"
									hClass="leading-[1.27] font-header font-bold mb-4 text-left"
									color="text-gray-brand-2 dark:text-gray-brand-4"
									customSize="lg:text-[30px] text-[24px]"
								>
									Export event models and generate Avro
								</SectionHeader>
								<SectionCopy
									pClass="mb-4 leading-6 text-left text-body-light dark:text-body-dark"
									textSize="text-[18px]"
								>
									Export your model schemas in JSON format for collaboration, reuse, and
									safekeeping. After you’ve defined datatype schemas at the model and model object
									levels, generate Avro to accelerate development. Integrate the exported assets
									with your event-driven ecosystem.
								</SectionCopy>
							</Section>
						</div>
					</TabContentItem>
					<TabContentItem id={3} {activeTabValue}>
						<div class="flex justify-center items-center px-4">
							<Section name="feature" sectionClass="lg:mt-0">
								<SectionCopy
									pClass="text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold"
									textSize="text-sm"
								>
									Operate
								</SectionCopy>
								<SectionHeader
									tag="h2"
									hClass="leading-[1.27] font-header font-bold mb-4 text-left"
									color="text-gray-brand-2 dark:text-gray-brand-4"
									customSize="lg:text-[30px] text-[24px]"
								>
									Visualize data
								</SectionHeader>
								<SectionCopy
									pClass="mb-4 leading-6 text-left text-body-light dark:text-body-dark"
									textSize="text-[18px]"
								>
									Integrate with data in motion systems like Apache Kafka. Visualize real-time data
									analytics from operations.
								</SectionCopy>
							</Section>
						</div>
					</TabContentItem>
				</TabWrapper>
			</Column>
		</Row>
	</Container>
</main>
