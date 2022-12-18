<script lang='ts'>
    import classNames from 'classnames';
    import Container from './Container.svelte';
    import SectionLabel from './typography/SectionLabel.svelte';
    import A from './typography/A.svelte';
    import Row from './Row.svelte';
    import Column from './Column.svelte';
    import EnablesDigitalTransformation from './assets/images/homepage/crossFunctionalGraphic.svg';
    import EmpowerDevelopers from './assets/images/homepage/empowerDev.svg';
    import LeverageDataStructure from './assets/images/homepage/leverageDataStructure.svg';
    import PoorSoftwareQuality from './assets/images/homepage/poorSoftwareQuality.svg';
    let mainClass='bg-white dark:bg-dark-1 w-full h-auto z-10 relative pb-[128px]';
    export let gridClass:string='grid lg:grid-cols-2 grid-col-1 mt-[44px]';
    export let flexClass:string='flex justify-center items-center px-4';
    export let containerClass: string | undefined='container';
    import SeventyFive from './assets/images/homepage/infoSeventyFive.svg';
    import SeventyEight from './assets/images/homepage/infoSeventyEight.svg';
    import Eighty from './assets/images/homepage/infoEighty.svg';
    import FifyFive from './assets/images/homepage/infoFiftyFive.svg';
    import GradientDivider from './utils/GradientDivider.svelte';
    import FifteenOBg from './assets/images/homepage/fifteen-o-bg.svg';
    import Community from './assets/images/homepage/ico-community.svg';
    import SearchDocs from './assets/images/homepage/ico-search-docs.svg';
    import Icon from './Icon.svelte';
    import ArrowRight from './icons/ArrowRight.svelte';
</script>

<main class={mainClass}>
    <Container class={classNames('pb-[128px]', containerClass)}>
        <SectionLabel>
            The value of oNote
        </SectionLabel>
        <Row class={classNames('mt-[44px]',gridClass)}>
            <Column class='flex order-first mx-auto'>
                <img src={EnablesDigitalTransformation} class="md:w-[566.75px] w-[343px]" width="auto" alt="Enables digital transformation">
            </Column>
            <Column>
                <div class="lg:mt-0 mt-8">
                    <h2 class="font-header text-gray-brand-2 dark:text-gray-brand-4 lg:text-[30px] text-[24px] leading-[1.27] font-bold mb-4 lg:pl-4 lg:text-left text-center">
                        Enable digital transformation
                    </h2>
                    <div class="text-body-light dark:text-body-dark text-[18px] leading-normal lg:pl-4 lg:text-left text-center">
                        oNote promotes collaboration between technical and non-technical teams by creating shared visual models that communicate project requirements and the overall business strategy.
                    </div>
                </div>
            </Column>
        </Row>
        <Row class={classNames('mt-[123px]',gridClass)}>
            <Column class='flex lg:order-last mx-auto'>
                <img src={EmpowerDevelopers} class="md:w-[450.22px] w-[343px]" width="auto" alt="Empower developers">
            </Column>
            <Column>
                <div class="lg:mt-0 mt-8">
                    <h2 class="font-header text-gray-brand-2 dark:text-gray-brand-4 lg:text-[30px] text-[24px] leading-[1.27] font-bold mb-4 lg:pl-4 lg:text-left text-center">
                        Empower developers
                    </h2>
                    <p class="text-[18px] leading-normal lg:pl-4 lg:text-left text-center">
                        oNote empowers developers by providing clear requirements as well as an accurate picture of the business needs, logic, and processes prior to coding.
                    </p>
                </div>
            </Column>
        </Row>
        <Row class={classNames('mt-[123px]',gridClass)}>
            <Column class='flex order-first mx-auto'>
                <img src={LeverageDataStructure} class="md:w-[311.42px] w-[213px]" width="213" alt="Leverage data structure">
            </Column>
            <Column>
                <div class="lg:mt-0 mt-8">
                    <h2 class="font-header text-gray-brand-2 dark:text-gray-brand-4 lg:text-[30px] text-[24px] leading-[1.27] font-bold mb-4 lg:pl-4 lg:text-left text-center">
                        Leverage data structure
                    </h2>
                    <p class="text-[18px] leading-normal lg:pl-4 lg:text-left text-center">
                        oNote allows teams to leverage data structure to jumpstart other applications and projects you may have.
                    </p>
                </div>
            </Column>
        </Row>
    </Container>
    <div class="bg-black dark:bg-white bg-opacity-[4%] dark:bg-opacity-[4%]">
        <Container class={classNames('py-[60px]',containerClass)}>
            <Row class={classNames('lg:mt-[44px] mb-8 lg:py-[69px]',gridClass)}>
                <Column class="flex-col lg:order-last mx-auto mt-8 lg:mt-0 px-4">
                    <img src={PoorSoftwareQuality} class="w-[640px]" width="auto" alt="Poor software quality">
                </Column>
                <Column class={classNames('order-first lg:pr-12 lg:-mt-4 px-4')}>
                    <h2 class="font-header text-gray-brand-2 dark:text-gray-brand-4 lg:text-[24px] text-[24px] leading-[1.27] font-bold mb-4 lg:pl-4 lg:text-left text-center">
                        Poor software design costs a lot
                    </h2>
                    <div class="text-base leading-normal lg:pl-4 lg:text-left text-center">
                        <p class="mb-4">For 2020, the Consortium for Information & Software Quality (CISQ) determined the total Cost of Poor Software Quality (CPSQ) in the US is $2.08 trillion (T). The largest contributor to CPSQ is operational software failures. For 2020 estimated that it is ~$1.56 T, a 22% growth over 2 years – but that could be underestimated given the meteoric rise in cybersecurity failures, and that many failures go unreported. The underlying cause is primarily unmitigated flaws in the software.</p>
                        <p class="mb-4">The next largest contributor to CPSQ in unsuccessful development projects totaling $260 billion (B), which rose by 46% since 2018. The project failure rate has been steady at ~19% for over a decade. The underlying causes are varied, but one consistent theme has been the lack of attention to quality.</p>
                        <p class="mb-4">Source: <A href='https://www.it-cisq.org/cisq-files/pdf/CPSQ-2020-report.pdf'>The Cost of Poor Software Quality in the US: A 2020 Report</A></p>
                    </div>
                </Column>
            </Row>
            <Row class='grid xl:grid-cols-4 lg:grid-cols-2 md:grid-cols-2 grid-col-1 lg:mt-8 mt-16 pt-8'>
                <Column class="flex-col lg:order-last order-first mx-auto">
                    <div class="mb-8 flex items-center justify-center">
                        <img src={SeventyFive} class="w-[165px]" width="auto" alt="75%">
                    </div>
                    <div class="xl:py-4 pt-0 pb-16">
                        <div class="text-[18px] leading-normal text-center px-12">
                            75% admit that their projects are typically destined to fail.
                        </div>
                    </div>
                </Column>
                <Column class="flex-col lg:order-last order-first mx-auto">
                    <div class="mb-8 flex items-center justify-center">
                        <img src={SeventyEight} class="w-[165px]" width="auto" alt="78%">
                    </div>
                    <div class="xl:py-4 pt-0 pb-16">
                        <div class="text-[18px] leading-normal text-center px-12">
                            78% feel their organization is typically out of sync with project requirements. Stakeholders should be more engaged in project requirements.
                        </div>
                    </div>
                </Column>
                <Column class="flex-col lg:order-last order-first mx-auto">
                    <div class="mb-8 flex items-center justify-center">
                        <img src={Eighty} class="w-[165px]" width="auto" alt="80%">
                    </div>
                    <div class="xl:py-4 pt-0 pb-16">
                        <div class="text-[18px] leading-normal text-center px-12">
                            80% admit they spend at least half their time on rework.
                        </div>
                    </div>
                </Column>
                <Column class="flex-col lg:order-last order-first mx-auto">
                    <div class="mb-8 flex items-center justify-center">
                        <img src={FifyFive} class="w-[165px]" width="auto" alt="55%">
                    </div>
                    <div class="xl:py-4 pt-0 pb-16">
                        <div class="text-[18px] leading-normal text-center px-12">
                            Only 55% feel that the business objectives of their projects are clear to them.
                        </div>
                    </div>
                </Column>
            </Row>
        </Container>
        <GradientDivider/>
    </div>
    <Container>
        <SectionLabel class='text-gray-primary dark:text-gray-brand-2 opacity-100 dark:opacity-60 font-bold lg:py-16 py-4 ml-4 text-left'>
            Getting Started
        </SectionLabel>
        <Row class='bg-white dark:bg-dark-1 shadow-lg lg:p-16 p-8 mx-4 rounded-lg grid gap-4 lg:grid-cols-3 grid-col-1 bg-bottom bg-no-repeat bg-contain' style='background-image: url({FifteenOBg})'>
            <div class='lg:col-span-1'>
                <Column class={classNames('h-full',flexClass)}>
                    <div class="lg:mt-0 mt-8">
                        <h2 class="text-gray-brand-2 dark:text-gray-brand-4 lg:text-[24px] text-[24px] leading-[1.27] font-bold font-header mb-4 lg:pl-4 lg:text-left text-center">
                            Design right from the start
                        </h2>
                    </div>
                </Column>
            </div>
            <div class='lg:col-span-1'>
                <div class="p-4 h-full bg-white bg-opacity-[72%] dark:bg-dark-1 dark:bg-opacity-[72%] rounded-md border border-border-light dark:border-border-dark">
                    <img src={Community} class="lg:m-4 mx-auto my-0" width="28" alt="Join the oNote community">
                    <div class="lg:mt-0 mt-8">
                        <div class="flex flex-col">
                            <h4 class="min-h-[60px] text-gray-brand-2 dark:text-gray-brand-4 lg:text-[24px] text-[24px] leading-[1.27] font-header font-bold mb-4 lg:pl-4 lg:text-left text-center">
                                Join the oNote community
                            </h4>
                            <div class="min-h-[112px] flex text-base leading-normal lg:pl-4 lg:text-left text-center">
                                Learn from others on their use cases and where they’ve levaraged oNote.
                            </div>
                            <div class="flex lg:text-left text-center pl-4">
                                <A href='/use-cases'>
                                    Go to Use Cases<Icon name='arrow-right' size={12} iconColor='fill-current' class='stroke-2 ml-1'><ArrowRight/></Icon>
                                </A>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class='lg:col-span-1'>
                <div class="p-4 h-full bg-white bg-opacity-[72%] dark:bg-dark-1 dark:bg-opacity-[72%] rounded-md border border-border-light dark:border-border-dark">
                    <img src={SearchDocs} class="lg:m-4 mx-auto my-0" width="28" alt="Join the oNote community">
                    <div class="lg:mt-0 mt-8">
                        <div class="flex flex-col">
                            <h4 class="min-h-[60px] text-gray-brand-2 dark:text-gray-brand-4 lg:text-[24px] text-[24px] leading-[1.27] font-header font-bold mb-4 lg:pl-4 lg:text-left text-center">
                                Read the docs
                            </h4>
                            <div class="min-h-[112px] flex text-base leading-normal lg:pl-4 lg:text-left text-center">
                                Get a headstart on how to go in-depth with oNote.
                            </div>
                            <div class="flex lg:text-left text-center pl-4">
                                <A href='/docs'>
                                    Go to Docs<Icon name='arrow-right' size={12} iconColor='fill-current' class='stroke-2 ml-1'><ArrowRight/></Icon>
                                </A>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Row>
    </Container>
</main>


