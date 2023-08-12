<script lang="ts">
  import TabWrapper from './TabWrapper.svelte';
  import TabHead from './TabHead.svelte';
  import TabHeadItem from './TabHeadItem.svelte';
  import TabContentItem from './TabContentItem.svelte';
  import Section from '../Section.svelte';
  import Container from '../Container.svelte';
  import SectionHeader from '../typography/SectionHeader.svelte';
  import SectionCopy from '../typography/SectionCopy.svelte';

  let activeTabValue = 1;
  const handleClick = (tabValue: number) => () => {
    activeTabValue = tabValue;
  };
  import { fly } from 'svelte/transition';
  import { sineIn } from 'svelte/easing';
  let transitionParams = {
    x: 32,
    duration: 200,
    easing: sineIn,
    opacity: 100
  };
  let fadeOut = {
    duration: 0,
    opacity: 0
  };
</script>

<Container class="my-16">
  <TabWrapper>
    <TabHead ulClass="flex list-none pb-0 mt-[-28px] lg:ml-4">
      <TabHeadItem
        id={1}
        on:click={handleClick(1)}
        {activeTabValue}
        name="Design"
        class="featureButtons featureTabs activeTabValue"
      />
      <TabHeadItem
        id={2}
        on:click={handleClick(2)}
        {activeTabValue}
        name="Implemant"
        class="featureButtons featureTabs activeTabValue"
      />
      <TabHeadItem
        id={3}
        on:click={handleClick(3)}
        {activeTabValue}
        name="Operate"
        class="featureButtons featureTabs activeTabValue"
      />
    </TabHead>
    <TabContentItem id={1} {activeTabValue}>
      <div
        class="flex justify-center items-center px-4"
        in:fly={transitionParams}
        out:fly={fadeOut}
      >
        <Section name="feature" sectionClass="lg:mt-0">
          <SectionCopy
            pClass="text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold"
            textSize="text-sm"
          >
            Design
          </SectionCopy>
          <SectionHeader
            tag="h2"
            hClass="leading-[1.27] font-copy font-bold mb-4 text-left"
            color="text-gray-brand-2 dark:text-gray-brand-4"
            customSize="lg:text-[30px] text-[24px]"
          >
            Build models
          </SectionHeader>
          <SectionCopy
            pClass="mb-4 leading-6 text-left text-body-light dark:text-body-dark"
            textSize="text-[18px]"
          >
            Build valid event-driven models that design a blueprint for all personas.
          </SectionCopy>
          <SectionCopy
            pClass="leading-6 text-left text-body-light dark:text-body-dark"
            textSize="text-[18px]"
          >
            Define datatype schemas at the model and model object levels. Reference parent schemas
            for convenient reuse.
          </SectionCopy>
        </Section>
      </div>
    </TabContentItem>
    <TabContentItem id={2} {activeTabValue}>
      <div
        class="flex justify-center items-center px-4"
        in:fly={transitionParams}
        out:fly={fadeOut}
      >
        <Section name="feature" sectionClass="lg:mt-0">
          <SectionCopy
            pClass="text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold"
            textSize="text-sm"
          >
            Implement
          </SectionCopy>
          <SectionHeader
            tag="h2"
            hClass="leading-[1.27] font-copy font-bold mb-4 text-left"
            color="text-gray-brand-2 dark:text-gray-brand-4"
            customSize="lg:text-[30px] text-[24px]"
          >
            Export event models and generate Avro
          </SectionHeader>
          <SectionCopy
            pClass="mb-4 leading-6 text-left text-body-light dark:text-body-dark"
            textSize="text-[18px]"
          >
            Export your model schemas in JSON format for collaboration, reuse, and safekeeping.
            After you’ve defined datatype schemas at the model and model object levels, generate
            Avro to accelerate development. Integrate the exported assets with your event-driven
            ecosystem.
          </SectionCopy>
        </Section>
      </div>
    </TabContentItem>
    <TabContentItem id={3} {activeTabValue}>
      <div
        class="flex justify-center items-center px-4"
        in:fly={transitionParams}
        out:fly={fadeOut}
      >
        <Section name="feature" sectionClass="lg:mt-0">
          <SectionCopy
            pClass="text-stack-brand-500 dark:text-stack-brand-500 uppercase font-bold"
            textSize="text-sm"
          >
            Operate
          </SectionCopy>
          <SectionHeader
            tag="h2"
            hClass="leading-[1.27] font-copy font-bold mb-4 text-left"
            color="text-gray-brand-2 dark:text-gray-brand-4"
            customSize="lg:text-[30px] text-[24px]"
          >
            Visualize data
          </SectionHeader>
          <SectionCopy
            pClass="mb-4 leading-6 text-left text-body-light dark:text-body-dark"
            textSize="text-[18px]"
          >
            Integrate with data in motion systems like Apache Kafka. Visualize real-time data
            analytics from operations.
          </SectionCopy>
        </Section>
      </div>
    </TabContentItem>
  </TabWrapper>
</Container>

<style>
  .featureButtons.active {
    border-color: #2fb6e1;
  }
  .featureButtons.active h2 {
    color: #2fb6e1;
    transition-property: color, background-color, border-color, text-decoration-color, fill, stroke,
      opacity, box-shadow, transform, filter, backdrop-filter;
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
    content: '';
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
    transition-property: color, background-color, border-color, text-decoration-color, fill, stroke,
      opacity, box-shadow, transform, filter, backdrop-filter;
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
