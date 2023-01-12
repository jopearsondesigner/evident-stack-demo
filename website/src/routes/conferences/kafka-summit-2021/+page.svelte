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
    let Kafka2021 = 'https://www.youtube.com/embed/nx9LH20FnzU';
</script>

<Article>
    <ArticleHead>
        <Row class='sm:container px-4 grid lg:grid-cols-6 px-4 py-16'>
            <Column class='lg:col-start-2 col-span-4'>
                <Section name=blog>
                    <SectionHeader tag=h1 class='text-body-2 dark:text-body-4' color='text-gray-brand-2' customSize='lg:text-[30px] text-[24px]'>
                        Kafka Summit 2021
                    </SectionHeader>
                    <YouTube src={Kafka2021}/>
                </Section>
            </Column>
        </Row>
    </ArticleHead>
    <ArticleBody>
        <Row class='sm:container px-4 grid lg:grid-cols-6 pb-16'>
            <Column class="lg:col-start-2 col-span-4">
                <Section name=blog>
                    <SectionCopy classes='my-4'>
                        It’s a dream as old as business computing: the ability to create a graphical model and then to deploy it as a working information system. Many attempts to realize this dream have come and gone with varying degrees of success, from visual programming languages like Visual Basic and Scratch, to business workflow systems like BPMN and its proprietary commercial variants, to engineering-focused systems like UML.
                    </SectionCopy>
                    <SectionCopy classes='my-4'>
                        But let’s face it: most low-code and model-based application development tools fall far short of the needs of modern software development teams. At best, they’re useful for rapidly testing ideas and creating prototypes. At worst, they’re used by “”citizen coders”” to cynically circumvent good engineering practices, with IT operations left holding the bag of operating, securing, and scaling black-box applications that cut against modern DevSecOps practices.
                    </SectionCopy>
                    <SectionCopy classes='my-4'>
                        Event-driven application architecture, enabled by infrastructure like Kafka and its ecosystem, has the potential to dramatically advance toward the age-old, model-driven and low-code dream. But what would an event-centric and developer-friendly low-code look like?
                    </SectionCopy>
                    <SectionCopy classes='my-4'>
                        This talk will outline strategies for low-code and model-driven development based on Event Modeling. We’ll explore how event-driven application architecture provides a simple yet robust framework for generating DevSecOps-friendly code for the UI, for the web services layer, and for event-processing.
                    </SectionCopy>
                </Section>
            </Column>
        </Row>
    </ArticleBody>
</Article>
