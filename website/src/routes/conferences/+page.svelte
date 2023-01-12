<script lang='ts'>
    import  Navbar from '$lib/components/websitenavbar/Navbar.svelte';
    import  NavWrapper from '$lib/components/websitenavbar/NavWrapper.svelte';
    import  Nav from '$lib/components/websitenavbar/Nav.svelte';
    import  NavUl from '$lib/components/websitenavbar/NavUl.svelte';
    import  NavLi from '$lib/components/websitenavbar/NavLi.svelte';
    import Container from '$lib/components/Container.svelte';
    import Section from '$lib/components/Section.svelte';
    import SectionHeader from '$lib/components/typography/SectionHeader.svelte';
    import SectionCopy from '$lib/components/typography/SectionCopy.svelte';
    import SectionLabel from '$lib/components/typography/SectionLabel.svelte'
    import Row from '$lib/components/Row.svelte';
    import A from '$lib/components/typography/A.svelte';
    import Column from '$lib/components/Column.svelte';
    import Button from '$lib/components/Button.svelte';
    import Icon from '$lib/components/Icon.svelte';
    import ArrowRight from '$lib/components/icons/ArrowRight.svelte';
    import Article from '$lib/components/blog/Article.svelte';
    import FeaturedBadge from '$lib/components/blog/FeaturedBadge.svelte';
    import BlogCta from '$lib/components/blog/BlogCta.svelte';
    import FeaturedBlogImage from '$lib/components/assets/images/blog/conference-photo-2021.png';
    import BlogImage1 from '$lib/components/assets/images/blog/conference-photo-1.png';
</script>

<Navbar navbarClass='w-full bg-black bg-opacity-90 px-4 py-3 sticky top-0 z-10 shadow-md'>
    <NavWrapper navDivClass='sm:container'>
        <Nav website navWebClass='block'>
            <NavUl class='flex justify-center items-center pb-0 mr-4 space-x-8 list-none'>
                <NavLi href='blog' customColor='text-body-dark hover:text-body-light transition duration-200 ease-in-out'>Blog</NavLi>
                <NavLi href='webinars' customColor='text-body-dark hover:text-body-light transition duration-200 ease-in-out'>Webinars</NavLi>
                <NavLi customColor='text-body-light hover:text-body-light transition duration-200 ease-in-out'>Conferences</NavLi>
            </NavUl>
        </Nav>
    </NavWrapper>
</Navbar>
<Container>
    <SectionLabel>Evident conference talks</SectionLabel>
    <Section name=blog divClass='md:pt-16 pt-0 pb-[136px]'>
        <Row customCols='lg:grid-cols-6'>
            <Column class='lg:block hidden order-first block m-4 pl-4 ml-0' customSpan='lg:col-span-4 col-span-2'>
                <FeaturedBadge/>
                <A href='/conferences/kafka-summit-2021'><img src={FeaturedBlogImage} alt='Kafka Summit 2021' class='object-cover w-full h-full' width=100% height=100%></A>
            </Column>
            <Column class='order-first m-4 lg:pl-4 lg:pl-4 pl-4 ml-0' customSpan='lg:col-span-2 col-span-2'>
                <Article>
                    <FeaturedBadge divClass='lg:hidden'/>
                    <div class="lg:hidden">
                        <A href='/conferences/kafka-summit-2021'><img src={FeaturedBlogImage} alt='Kafka Summit 2021' class='object-cover'></A>
                    </div>
                    <p class="text-sm text-black text-opacity-90 dark:text-gray-primary uppercase font-bold lg:pt-0 mt-2 mb-4">
                        Conference
                    </p>
                    <Section name=blog>
                        <SectionHeader tag=h2 customSize="text-[30px]">
                            <A color='text-gray-brand-2 hover:text-gray-brand-1' href='/conferences/kafka-summit-2021'>
                                Kafka Summit 2021
                            </A>
                        </SectionHeader>
                        <SectionCopy>
                            <div class='line-clamp-4'>
                                It’s a dream as old as business computing: the ability to create a graphical model and then to deploy it as a working information system. Many attempts to realize this dream have come and gone with varying degrees of success, from visual programming languages like Visual Basic and Scratch, to business workflow systems like BPMN and its proprietary commercial variants, to engineering-focused systems like UML.
                            </div>
                            <A href='/conferences/kafka-summit-2021'>Read More</A>
                        </SectionCopy>
                    </Section>
                    <Section name=blog sectionClass='h-full flex flex-col justify-end items-start' divClass='w-full'>
                        <div class="text-sm text-gray-brand-4 mt-4">
                            2021-09-15
                        </div>
                        <BlogCta label='Join the community.'>
                            <Button gradient color='brandDesignPrimary' size='sm' label='Get Started' class='flex items-center' href='#'>
                                <Icon name="arrow-right" slot=icon class='inline-flex ml-1 stroke-2' iconColor='fill-current text-white' size={12}><ArrowRight/></Icon>
                            </Button>
                        </BlogCta>
                    </Section>
                </Article>
            </Column>
            <Column class='m-4 border-b border-border-light dark:border-border-dark' customSpan='col-span-2'>
                <Article customMargin='mt-0'>
                    <A href='conferences/kafka-summit-2020'><img src={BlogImage1} alt='Event Modeling Explained' class='object-cover'></A>
                    <p class="text-sm text-black text-opacity-90 dark:text-gray-primary uppercase font-bold lg:pt-0 mt-2 mb-4">
                        Conference
                    </p>
                    <Section name=blog divClass='h-full'>
                        <SectionHeader tag=h2 customSize="text-[30px]">
                            <A color='text-gray-brand-2 hover:text-gray-brand-1' href='conferences/kafka-summit-2020'>
                                Kafka Summit 2020
                            </A>
                        </SectionHeader>
                        <SectionCopy>
                            <div class='line-clamp-2'>
                                Bobby’s presentation on Event Modeling at Kafka Summit 2020 mere days after the launching of the oNote product. In the talk, he builds an Event Model of a simple information system (including wire-framing the UI/UX experience), explores how to proceed from model to implementation using Kafka, shows you how to jump-start the implementation by generating code directly from the Event Model, and also how to track and measure the work of implementation by generating tasks directly from the Event Model.
                            </div>
                            <A href='conferences/kafka-summit-2020'>Read More</A>
                        </SectionCopy>
                        <div class="text-sm text-gray-brand-4 mt-4">
                            2020-08-25
                        </div>
                    </Section>
                </Article>
            </Column>
        </Row>
    </Section>
</Container>
