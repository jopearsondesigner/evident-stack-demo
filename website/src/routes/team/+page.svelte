<script lang='ts'>
    import Card from 'components/card/Card.svelte';
    import Container from 'components/Container.svelte';
    import Row from 'components/Row.svelte';
    import Column from 'components/Column.svelte';
    import Section from 'components/Section.svelte';
    import SectionHeader from 'components/typography/SectionHeader.svelte';
    import SectionCopy from 'components/typography/SectionCopy.svelte';
    import SectionLabel from 'components/typography/SectionLabel.svelte';
    import A from 'components/typography/A.svelte';
    import Bobby from 'components/assets/images/team/bobby.png';
    import Ryan from 'components/assets/images/team/ryan.png';
    import Mike from 'components/assets/images/team/mike.png';
    import Jo from 'components/assets/images/team/jo.png';
    import Kevin from 'components/assets/images/team/kevin.png';
    import Tracey from 'components/assets/images/team/tracey.png';
    import John from 'components/assets/images/team/john.png';
    import Anthony from 'components/assets/images/team/anthony.png';
    import Adam from 'components/assets/images/team/adam.png';
</script>

<div class="bg-black dark:bg-white bg-opacity-[4%] dark:bg-opacity-[4%] pb-24">
    <Container class='sm:container pb-4'>
        <SectionLabel>
            Evident team
        </SectionLabel>
    </Container>
    <Container class='sm:container w-full h-auto z-10 relative'>
        <Row class='grid px-4' customCols='lg:grid-cols-6'>
            <Column customSpan='lg:col-start-2 col-span-4'>
                <Section name='general'>
                    <SectionHeader tag='h2' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[30px] text-[24px]' color='text-gray-brand-2 dark:text-gray-brand-4'>
                        Meet the Evident team!
                    </SectionHeader>
                </Section>
            </Column>
        </Row>
    </Container>
    <Container class='sm:container lg:my-24 my-0'>
        <Row class='grid gap-12' customCols='lg:grid-cols-3 md:grid-cols-2 grid-cols-1'>
            <Column class='lg:grid-cols-1'>
                <Card img={Bobby} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            Bobby Calderwood
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            Founder & Sofware Engineer
                        </SectionCopy>
                        <SectionCopy pClass='leading-normal text-center px-4 mb-10 text-body-light dark:text-body-dark' textSize='text-medium'>
                            Bobby drives a truck and develops software. A true renaissance man, no?
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
            <Column class='lg:grid-cols-2'>
                <Card img={Ryan} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            Ryan Murry
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            GET TITLE OF ROLE
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
            <Column class='lg:grid-cols-3'>
                <Card img={Mike} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            Mike Shearer
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            Lead Software Engineer
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
            <Column class='lg:grid-cols-1'>
                <Card img={Jo} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            Jo Pearson
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            Lead Designer
                        </SectionCopy>
                        <SectionCopy pClass='leading-normal text-center px-4 mb-10 text-body-light dark:text-body-dark' textSize='text-medium'>
                            Jo’s Love Languages include Words of Affirmation, State-of-the-art Apps, and Sweet Design.
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
            <Column class='lg:grid-cols-2'>
                <Card img={Kevin} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            Kevin Calderwood
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            Advisor
                        </SectionCopy>
                        <SectionCopy pClass='leading-normal text-center px-4 mb-10 text-body-light dark:text-body-dark' textSize='text-medium'>
                            Kevin combined his two passions for this oNote project: family and investing.
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
            <Column class='lg:grid-cols-3'>
                <Card img={Tracey} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            Tracey Mustacchio
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            Advisor
                        </SectionCopy>
                        <SectionCopy pClass='leading-normal text-center px-4 mb-10 text-body-light dark:text-body-dark' textSize='text-medium'>
                            Technology lover and marketer by day, dessert connoisseur and book critic by night.
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
            <Column class='lg:grid-cols-1'>
                <Card img={John} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            John Kealey
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            Advidor
                        </SectionCopy>
                        <SectionCopy pClass='leading-normal text-center px-4 mb-10 text-body-light dark:text-body-dark' textSize='text-medium'>
                            When John isn’t advising, he’s out boating around on the lake with his wife.
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
            <Column class='lg:grid-cols-2'>
                <Card img={Anthony} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            Anthony Archibald
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            Advisor
                        </SectionCopy>
                        <SectionCopy pClass='leading-normal text-center px-4 mb-10 text-body-light dark:text-body-dark' textSize='text-medium'>
                            Anthony loves mountain biking and skiing with his 4 daughters.
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
            <Column class='lg:grid-cols-3'>
                <Card img={Adam} size='sm' cardClass='min-h-full mx-6 bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-10 mt-10 mx-auto' width={155}>
                    <Section name='general'>
                        <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                            Adam Dymitruk
                        </SectionHeader>
                        <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                            Distinguished Technical Advisor
                        </SectionCopy>
                        <SectionCopy pClass='leading-normal text-center px-4 mb-10 text-body-light dark:text-body-dark' textSize='text-medium'>
                            Developed Event Modeling as a way to build information systems predictably.
                        </SectionCopy>
                    </Section>
                </Card>
            </Column>
        </Row>
    </Container>
</div>
<Container>
    <Row class='grid my-16 px-4' customCols='lg:grid-cols-6'>
        <Column customSpan='lg:col-start-2 col-span-4'>
            <Section name='general'>
                <SectionHeader tag='h2' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[30px] text-[24px]' color='text-gray-brand-2 dark:text-gray-brand-4'>
                    Evident Stack is a product of Evident Systems LLC.
                </SectionHeader>
                <SectionCopy class='my-4'>
                    <A class='transition duration-300 ease-in' href='https://www.linkedin.com/in/bobbycalderwood/'>Bobby Calderwood</A> founded Evident Systems LLC in 2018 to help companies build event-based information systems. He has <A class='transition duration-300 ease-in' href='https://www.youtube.com/watch?v=B1-gS0oEtYc'>spoken at conferences</A> and <A class='transition duration-300 ease-in' href='https://www.confluent.io/blog/toward-functional-programming-analogy-microservices/'>written about</A> achieving better software architecture by focusing on business events.
                </SectionCopy>
            </Section>
        </Column>
    </Row>
</Container>
