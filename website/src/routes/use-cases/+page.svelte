<script lang="ts">
  import Container from '$components/Container.svelte';
  import Row from '$components/Row.svelte';
  import Column from '$components/Column.svelte';
  import Section from '$components/Section.svelte';
  import SectionCopy from '$components/typography/SectionCopy.svelte';
  import SectionLabel from '$components/typography/SectionLabel.svelte';
</script>

<Container class="sm:container pb-4">
  <Section name="general" class="flex items-center px-4">
    <SectionLabel>Use cases</SectionLabel>
  </Section>
</Container>
<Container class="sm:container w-full h-auto relative">
  <Row class="grid px-4" customCols="lg:grid-cols-6">
    <Column
      class="flex justify-center items-center px-4 h-screen"
      customSpan="lg:col-start-2 col-span-4"
    >
      <Section name="general">
        <SectionCopy classes="font-bold" textSize="text-[30px]">Coming soon!</SectionCopy>
      </Section>
    </Column>
  </Row>
</Container>
