<script lang='ts'>
    import Card from './Card.svelte';
    import Container from '../Container.svelte';
    import Row from '../Row.svelte';
    import Column from '../Column.svelte';
    import Section from '../Section.svelte';
    import SectionHeader from '../typography/SectionHeader.svelte';
    import SectionCopy from '../typography/SectionCopy.svelte';
    import Cash from '../assets/images/team/cash.png';
</script>

<Container class='container my-16'>
    <Row class='grid lg:grid-cols-3 md:grid-cols-2 grid-cols-1 gap-4'>
        <Column class='lg:mt-0 px-4'>
            <Card img={Cash} size='sm' cardClass='group bg-white dark:bg-dark-1 border border-border-light dark:border-border-dark p-4 rounded-lg shadow-lg' imgClass='mb-8 mt-4 mx-auto' width={255}>
                <Section name='general'>
                    <SectionHeader tag='h4' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[24px] text-[24px] ' color='text-gray-brand-2 dark:text-gray-brand-4'>
                        Cash Monestine
                    </SectionHeader>
                    <SectionCopy pClass='uppercase text-gray-primary dark:text-gray-brand-4 leading-[1.27] font-bold font-header mb-2 text-center' textSize='text-xs'>
                        Sales
                    </SectionCopy>
                    <SectionCopy pClass='leading-normal text-center px-4 mb-4 text-body-light dark:text-body-dark' textSize='text-medium'>
                        Cash loves long walks on the beach, delicious steak dinners and Event Modeling.
                    </SectionCopy>
                </Section>
            </Card>
        </Column>
    </Row>
</Container>

