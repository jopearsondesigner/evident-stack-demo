<script lang='ts'>
    import Article from '$lib/components/blog/Article.svelte';
    import ArticleHead from '$lib/components/blog/ArticleHead.svelte';
    import ArticleBody from '$lib/components/blog/ArticleBody.svelte';
    import Row from '$lib/components/Row.svelte';
    import Column from '$lib/components/Row.svelte';
    import Section from '$lib/components/Section.svelte';
    import SectionHeader from '$lib/components/typography/SectionHeader.svelte';
    import SectionCopy from '$lib/components/typography/sectionCopy.svelte';
    import YouTube from '$lib/components/utils/YouTube.svelte';
    let WebinarVideo = 'https://www.youtube.com/embed/O-shbyRY9xc?feature=oembed';
</script>

<Article>
    <ArticleHead>
        <Row class='sm:container px-4 grid lg:grid-cols-6 px-4 py-16'>
            <Column class='lg:col-start-2 col-span-4'>
                <Section name=blog>
                    <SectionHeader tag=h1 class='text-body-2 dark:text-body-4' color='text-gray-brand-2' customSize='lg:text-[30px] text-[24px]'>
                        Accelerate from Event Model to implementation with oNote code-generation
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
                        <ul class="list-disc">
                            <li>Building a simple yet business-valuable Event Model</li>
                            <li>Defining data schemas for Events</li>
                            <li>Building request and response schemas for Commands and Read Models</li>
                            <li>Generating Avro Event schemas, and uploading to Confluent Schema Registry</li>
                            <li>Generating Avro Protocol for Commands and Read Models, and generating a gRPC implementation scaffold</li>
                            <li>Implementing the business logic with Kafka and Kafka Streams</li>
                        </ul>
                    </SectionCopy>
                </Section>
            </Column>
        </Row>
    </ArticleBody>
</Article>
