<script lang='ts'>
  import TabWrapper from './TabWrapper.svelte';
  import TabHead from './TabHead.svelte';
  import TabHeadItem from './TabHeadItem.svelte';
  import TabContentItem from './TabContentItem.svelte';
  import Section from '../Section.svelte';
  import Container from '../Container.svelte';
  import SectionHeader from '../typography/SectionHeader.svelte';
  import SectionCopy from '../typography/SectionCopy.svelte';
  let activeTabValue = 1;
  const handleClick = (tabValue:number) => () => {
    activeTabValue = tabValue;
  };
</script>

<Container class='my-16'>
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
</Container>