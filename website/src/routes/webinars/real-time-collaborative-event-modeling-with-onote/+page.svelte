<script lang='ts'>
    import Article from 'components/blog/Article.svelte';
    import ArticleHead from 'components/blog/ArticleHead.svelte';
    import ArticleBody from 'components/blog/ArticleBody.svelte';
    import Row from 'components/Row.svelte';
    import Column from 'components/Row.svelte';
    import Section from 'components/Section.svelte';
    import SectionHeader from 'components/typography/SectionHeader.svelte';
    import SectionCopy from 'components/typography/SectionCopy.svelte';
    import YouTube from 'components/utils/YouTube.svelte';
    let WebinarVideo = 'https://www.youtube.com/embed/teFHMHsUSZ4?feature=oembed';
</script>

<Article>
    <ArticleHead>
        <Row class='sm:container px-4 grid lg:grid-cols-6 px-4 py-16'>
            <Column class='lg:col-start-2 col-span-4'>
                <Section name=blog>
                    <SectionHeader tag=h1 class='text-body-2 dark:text-body-4' color='text-gray-brand-2' customSize='lg:text-[30px] text-[24px]'>
                        Real-time Collaborative Event Modeling with oNote
                    </SectionHeader>
                    <YouTube src={WebinarVideo}/>
                </Section>
            </Column>
        </Row>
    </ArticleHead>
    <ArticleBody>
        <Row class='sm:container mx-auto px-4 grid lg:grid-cols-6 pb-16'>
            <Column class="lg:col-start-2 col-span-4">
                <Section name=blog>
                    <SectionCopy classes='my-4'>
                        This webinar will cover oNote’s Real-Time Collaboration. Attendees will have the opportunity to peek under the hood and see how it was built. We will also hear from Martin Kleppmann (Pre Recorded), Mike Shearer (lead Engineer), and Bobby Calderwood (Founder of oNote). The oNote team will walk through starting a session, joining a session, and allow everyone to see how the interactions in the environment look as well. This is an opportunity to see where the future of software design and development is headed.
                    </SectionCopy>
                </Section>
            </Column>
        </Row>
    </ArticleBody>
</Article>
