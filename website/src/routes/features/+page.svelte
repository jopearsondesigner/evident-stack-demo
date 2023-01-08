<script lang='ts'>
    import classNames from 'classnames';
    import Container from 'components/Container.svelte';
    import SectionLabel from 'components/typography/SectionLabel.svelte';
    import Row from 'components/Row.svelte';
    import Column from 'components/Column.svelte';
    import Section from 'components/Section.svelte';
    import SectionHeader from 'components/typography/SectionHeader.svelte';
    import SectionCopy from 'components/typography/SectionCopy.svelte';
    import TabWrapper from 'components/tabs/TabWrapper.svelte';
    import TabHead from 'components/tabs/TabHead.svelte';
    import TabHeadItem from 'components/tabs/TabHeadItem.svelte';
    import TabContentItem from 'components/tabs/TabContentItem.svelte';
    let mainClass:string = 'w-full h-auto z-10 relative';
    let containerClass = 'sm:container mx-auto lg:py-16 py-4';
    import FeaturesDesign from 'components/assets/images/features/featuresDesign.svg';
    import FeaturesImplement from 'components/assets/images/features/featuresImplement.svg';
    import FeaturesOperate from 'components/assets/images/features/featuresOperate.svg';
    import FeaturesLogo from 'components/assets/images/features/featureslogo.svg';
    let activeTabValue = 1;
    const handleClick = (tabValue:number) => () => {
        activeTabValue = tabValue;
    };
</script>

<main class={mainClass}>
    <Container class='sm:container mx-auto pb-4'>
        <SectionLabel>
            Evident Stack features
        </SectionLabel>
    </Container>
    <Container class={containerClass}>
        <Row class='grid lg:grid-cols-6'>
            <Column class='hidden lg:block px-4' customSpan='lg:col-span-3'>
                <TabHead ulClass='gap-4 sm:flex overflow-hidden -mb-px'>
                    <TabHeadItem id={1} on:click={handleClick(1)} {activeTabValue}
                    liClass='list-none border-[3px] border-gray-brand-4 rounded-lg shadow-lg w-full mb-4 sm:mb-0 max-w-[318px] mx-auto'
                    buttonClass='featureButtons h-full border-2 border-border-light rounded-[5px] shadow-md px-4 py-2 w-full transition ease-in-out duration-300'
                    hClass='font-header font-bold text-[30px] text-body-light dark:text-body-dark whitespace-nowrap my-2'
                    name='Design'>
                        <img src={FeaturesDesign} alt='Design' width='auto' class='mx-auto hidden sm:block'/>
                        <div class=" min-h-[40px]">
                            <p class="text-body-light dark:text-body-dark text-sm pb-0">Build models</p>
                        </div>
                    </TabHeadItem>
                    <TabHeadItem id={2} on:click={handleClick(2)} {activeTabValue}
                    liClass='list-none border-[3px] border-gray-brand-4 rounded-lg shadow-lg w-full mb-4 sm:mb-0 max-w-[318px] mx-auto'
                    buttonClass='featureButtons h-full border-2 border-border-light rounded-[5px] shadow-md px-4 py-2 w-full transition ease-in-out duration-300'
                    hClass='font-header font-bold text-[30px] text-body-light dark:text-body-dark whitespace-nowrap my-2'
                    name='Implement'>
                        <img src={FeaturesImplement} alt='Implement' width='auto' class='mx-auto hidden sm:block'/>
                        <div class=" min-h-[40px]">
                            <p class="text-body-light dark:text-body-dark text-sm pb-0">Export event models and generate Avro</p>
                        </div>
                    </TabHeadItem>
                    <TabHeadItem id={3} on:click={handleClick(3)} {activeTabValue}
                    liClass='list-none border-[3px] border-gray-brand-4 rounded-lg shadow-lg w-full mb-4 sm:mb-0 max-w-[318px] mx-auto'
                    buttonClass='featureButtons h-full border-2 border-border-light rounded-[5px] shadow-md px-4 py-2 w-full transition ease-in-out duration-300'
                    hClass='font-header font-bold text-[30px] text-body-light dark:text-body-dark whitespace-nowrap my-2'
                    name='Operate'>
                        <img src={FeaturesOperate} alt='Operate' width='auto' class='mx-auto hidden sm:block'/>
                        <div class=" min-h-[40px]">
                            <p class="text-body-light dark:text-body-dark text-sm pb-0">Visualize data</p>
                        </div>
                    </TabHeadItem>
                </TabHead>
                <div class="hidden lg:block">
                    <img src={FeaturesLogo} alt='Evident Stack' width='auto' class='mx-auto max-w-[418px]'/>
                </div>
            </Column>
            <Column class='mx-4 lg:col-span-3 px-4 mt-8 lg:mt-0 box-border border-t lg:border-l border-border-light dark:border-border-dark'>
            <TabWrapper>
                <TabHead ulClass='flex list-none pb-0 mt-[-28px] lg:ml-4'>
                    <TabHeadItem id={1} on:click={handleClick(1)} {activeTabValue} name='Design'/>
                    <TabHeadItem id={2} on:click={handleClick(2)} {activeTabValue} name='Implemant'/>
                    <TabHeadItem id={3} on:click={handleClick(3)} {activeTabValue} name='Operate'/>
                </TabHead>
                    <TabContentItem id={1} {activeTabValue}>
                        <div class='flex justify-center items-center px-4'>
                            <Section name='feature' sectionClass='lg:mt-0'>
                                <SectionCopy pClass='text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold' textSize='text-sm'>
                                    Design
                                </SectionCopy>
                                <SectionHeader tag='h2' hClass='leading-[1.27] font-copy font-bold mb-4 text-left' color='text-gray-brand-2 dark:text-gray-brand-4' customSize='lg:text-[30px] text-[24px]'>
                                    Build models
                                </SectionHeader>
                                <SectionCopy pClass='mb-4 leading-6 text-left text-body-light dark:text-body-dark' textSize='text-[18px]'>
                                    Build valid event-driven models that design a blueprint for all personas.
                                </SectionCopy>
                                <SectionCopy pClass='leading-6 text-left text-body-light dark:text-body-dark' textSize='text-[18px]'>
                                    Define datatype schemas at the model and model object levels. Reference parent schemas for convenient reuse.
                                </SectionCopy>
                            </Section>
                        </div>
                    </TabContentItem>
                    <TabContentItem id={2} {activeTabValue}>
                        <div class="flex justify-center items-center px-4">
                            <Section name='feature' sectionClass='lg:mt-0'>
                                <SectionCopy pClass='text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold' textSize='text-sm'>
                                    Implement
                                </SectionCopy>
                                <SectionHeader tag='h2' hClass='leading-[1.27] font-copy font-bold mb-4 text-left' color='text-gray-brand-2 dark:text-gray-brand-4' customSize='lg:text-[30px] text-[24px]'>
                                    Export event models and generate Avro
                                </SectionHeader>
                                <SectionCopy pClass='mb-4 leading-6 text-left text-body-light dark:text-body-dark' textSize='text-[18px]'>
                                    Export your model schemas in JSON format for collaboration, reuse, and safekeeping. After you’ve defined datatype schemas at the model and model object levels, generate Avro to accelerate development. Integrate the exported assets with your event-driven ecosystem.
                                </SectionCopy>
                            </Section>
                        </div>
                    </TabContentItem>
                    <TabContentItem id={3} {activeTabValue}>
                        <div class="flex justify-center items-center px-4">
                            <Section name='feature' sectionClass='lg:mt-0'>
                                <SectionCopy pClass='text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold' textSize='text-sm'>
                                    Operate
                                </SectionCopy>
                                <SectionHeader tag='h2' hClass='leading-[1.27] font-copy font-bold mb-4 text-left' color='text-gray-brand-2 dark:text-gray-brand-4' customSize='lg:text-[30px] text-[24px]'>
                                    Visualize data
                                </SectionHeader>
                                <SectionCopy pClass='mb-4 leading-6 text-left text-body-light dark:text-body-dark' textSize='text-[18px]'>
                                    Integrate with data in motion systems like Apache Kafka. Visualize real-time data analytics from operations.
                                </SectionCopy>
                            </Section>
                        </div>
                    </TabContentItem>
                </TabWrapper>
            </Column>
        </Row>
    </Container>
</main>

<style>
    .featureButtons.active {
        border-color: #2FB6E1;
    }
    .featureButtons.active h2 {
        color: #2FB6E1;
        transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 150ms;
        transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
        transition-duration: 300ms;
    }
    .featureTabs {
        position: relative;
        display: inline-block;
        color: #586e75;
        text-decoration: none;
        padding: 0.3em 1em 0;
    }
    .featureTabs::before {
        content: "";
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        z-index: -1;
        border-bottom: none;
        border-radius: 0.5em 0.5em 0 0;
        background-color: #e6e6e6;
        box-shadow: 0 1px white inset;
        transform: scale(1.2, 1.3) perspective(0.5em) rotateX(5deg);
        transform-origin: bottom left;
        transition-property: color, background-color, border-color,
        text-decoration-color, fill, stroke, opacity, box-shadow, transform,
        filter, backdrop-filter;
        transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
        transition-duration: 300ms;
    }
    .featureTabs.active {
        color: white;
        z-index: 2;
    }
    .featureTabs.active::before {
        background-color: #2fb6e1;
        border-bottom-width: 1px;
        border-color: #2fb6e1;
    }
</style>


