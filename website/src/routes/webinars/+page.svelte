<script lang='ts'>
    import  Navbar from 'components/websitenavbar/Navbar.svelte';
    import  NavWrapper from 'components/websitenavbar/NavWrapper.svelte';
    import  Nav from 'components/websitenavbar/Nav.svelte';
    import  NavUl from 'components/websitenavbar/NavUl.svelte';
    import  NavLi from 'components/websitenavbar/NavLi.svelte';
    import Container from 'components/Container.svelte';
    import Section from 'components/Section.svelte';
    import SectionHeader from 'components/typography/SectionHeader.svelte';
    import SectionCopy from 'components/typography/SectionCopy.svelte';
    import SectionLabel from 'components/typography/SectionLabel.svelte'
    import Row from 'components/Row.svelte';
    import A from 'components/typography/A.svelte';
    import Column from 'components/Column.svelte';
    import Button from 'components/Button.svelte';
    import Icon from 'components/Icon.svelte';
    import ArrowRight from 'components/icons/ArrowRight.svelte';
    import Article from 'components/blog/Article.svelte';
    import FeaturedBadge from 'components/blog/FeaturedBadge.svelte';
    import BlogCta from 'components/blog/BlogCta.svelte';
    import FeaturedBlogImage from 'components/assets/images/blog/webinar-mar-2022.png';
    import BlogImage1 from 'components/assets/images/blog/webinar-dec-2021.png';
    import BlogImage2 from 'components/assets/images/blog/webinar-nov-2021.png';
</script>

<Navbar navbarClass='w-full bg-black bg-opacity-90 px-4 py-3 sticky top-0 z-10 shadow-md'>
    <NavWrapper navDivClass='sm:container'>
        <Nav website navWebClass='block'>
            <NavUl class='flex justify-center items-center pb-0 mr-4 space-x-8 list-none'>
                <NavLi href='conferences' customColor='text-body-dark hover:text-body-light transition duration-200 ease-in-out'>Blog</NavLi>
                <NavLi customColor='text-body-light hover:text-body-light transition duration-200 ease-in-out'>Webinars</NavLi>
                <NavLi href='conferences' customColor='text-body-dark hover:text-body-light transition duration-200 ease-in-out'>Conferences</NavLi>
            </NavUl>
        </Nav>
    </NavWrapper>
</Navbar>
<Container>
    <SectionLabel>Evident webinars</SectionLabel>
    <Section name=blog divClass='md:pt-16 pt-0 pb-[136px]'>
        <Row customCols='lg:grid-cols-6'>
            <Column class='lg:block hidden order-first block m-4 pl-4 ml-0' customSpan='lg:col-span-4 col-span-2'>
                <FeaturedBadge/>
                <A href='/webinars/real-time-collaborative-event-modeling-with-onote'><img src={FeaturedBlogImage} alt='Real-time Collaborative Event Modeling with oNote' class='object-cover w-full h-full' width=100% height=100%></A>
            </Column>
            <Column class='order-first m-4 lg:pl-4 lg:pl-4 pl-4 ml-0' customSpan='lg:col-span-2 col-span-2'>
                <Article>
                    <FeaturedBadge divClass='lg:hidden'/>
                    <div class="lg:hidden">
                        <A href='/webinars/real-time-collaborative-event-modeling-with-onote'><img src={FeaturedBlogImage} alt='Real-time Collaborative Event Modeling with oNote' class='object-cover'></A>
                    </div>
                    <p class="text-sm text-black text-opacity-90 dark:text-gray-primary uppercase font-bold lg:pt-0 mt-2 mb-4">
                        Webinar
                    </p>
                    <Section name=blog>
                        <SectionHeader tag=h2 customSize="text-[30px]">
                            <A color='text-gray-brand-2 hover:text-gray-brand-1' href='/webinars/real-time-collaborative-event-modeling-with-onote'>
                                Real-time Collaborative Event Modeling with oNote
                            </A>
                        </SectionHeader>
                        <SectionCopy>
                            <div class='line-clamp-4'>
                                This webinar will cover oNote’s Real-Time Collaboration. Attendees will have the opportunity to peek under the hood and see how it was built. We will also hear from Martin Kleppmann (Pre Recorded), Mike Shearer (lead Engineer), and Bobby Calderwood (Founder of oNote). The oNote team will walk through starting a session, joining a session, and allow everyone to see how the interactions in the environment look as well. This is an opportunity to see where the future of software design and development is headed.
                            </div>
                            <A href='/webinars/real-time-collaborative-event-modeling-with-onote'>Read More</A>
                        </SectionCopy>
                    </Section>
                    <Section name=blog sectionClass='h-full flex flex-col justify-end items-start' divClass='w-full'>
                        <div class="text-sm text-gray-brand-4 mt-4">
                            2022-02-23
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
                    <A href='/webinars/accelerate-from-event-model-to-implementation-with-onote-code-generation-event-store'><img src={BlogImage1} alt='Event Modeling Explained' class='object-cover'></A>
                    <p class="text-sm text-black text-opacity-90 dark:text-gray-primary uppercase font-bold lg:pt-0 mt-2 mb-4">
                        Webinar
                    </p>
                    <Section name=blog divClass='h-full'>
                        <SectionHeader tag=h2 customSize="text-[30px]">
                            <A color='text-gray-brand-2 hover:text-gray-brand-1' href='/webinars/accelerate-from-event-model-to-implementation-with-onote-code-generation-event-store'>
                                Accelerate from Event Model to Implementation with oNote Code-generation & Event Store
                            </A>
                        </SectionHeader>
                        <SectionCopy>
                            <div class='line-clamp-2'>
                                Learn from some of the leading experts in software design and software development.
                            </div>
                            <A href='/webinars/accelerate-from-event-model-to-implementation-with-onote-code-generation-event-store'>Read More</A>
                        </SectionCopy>
                        <div class="text-sm text-gray-brand-4 mt-4">
                            2021-12-15
                        </div>
                    </Section>
                </Article>
            </Column>
            <Column class='m-4 border-b border-border-light dark:border-border-dark' customSpan='col-span-2'>
                <Article customMargin='mt-0'>
                    <A href='/webinars/accelerate-from-event-model-to-implementation-with-onote-code-generation'><img src={BlogImage2} alt='Event Modeling Explained' class='object-cover'></A>
                    <p class="text-sm text-black text-opacity-90 dark:text-gray-primary uppercase font-bold lg:pt-0 mt-2 mb-4">
                        Webinar
                    </p>
                    <Section name=blog divClass='h-full'>
                        <SectionHeader tag=h2 customSize="text-[30px]">
                            <A color='text-gray-brand-2 hover:text-gray-brand-1' href='/webinars/accelerate-from-event-model-to-implementation-with-onote-code-generation'>
                                Accelerate from Event Model to implementation with oNote code-generation
                            </A>
                        </SectionHeader>
                        <SectionCopy>
                            <div class='line-clamp-2'>
                                Learn from some of the leading experts in software design and software development.
                            </div>
                            <A href='/webinars/accelerate-from-event-model-to-implementation-with-onote-code-generation'>Read More</A>
                        </SectionCopy>
                        <div class="text-sm text-gray-brand-4 mt-4">
                            2021-11-10
                        </div>
                    </Section>
                </Article>
            </Column>
        </Row>
    </Section>
</Container>
