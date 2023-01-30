<script lang="ts">
	import Article from '$lib/components/blog/Article.svelte';
	import ArticleHead from '$lib/components/blog/ArticleHead.svelte';
	import ArticleBody from '$lib/components/blog/ArticleBody.svelte';
	import Row from '$lib/components/Row.svelte';
	import Column from '$lib/components/Row.svelte';
	import Section from '$lib/components/Section.svelte';
	import SectionHeader from '$lib/components/typography/SectionHeader.svelte';
	import SectionCopy from '$lib/components/typography/SectionCopy.svelte';
	import YouTube from '$lib/components/utils/YouTube.svelte';
	import A from '$lib/components/typography/A.svelte';
	let Kafka2020 = 'https://www.youtube.com/embed/ArMXNNzice4?feature=oembed';
</script>

<Article>
	<ArticleHead>
		<Row class="sm:container px-4 grid lg:grid-cols-6 px-4 py-16">
			<Column class="lg:col-start-2 col-span-4">
				<Section name="blog">
					<SectionHeader
						tag="h1"
						class="text-body-2 dark:text-body-4"
						color="text-gray-brand-2"
						customSize="lg:text-[30px] text-[24px]"
					>
						Kafka Summit 2020
					</SectionHeader>
					<YouTube src={Kafka2020} />
				</Section>
			</Column>
		</Row>
	</ArticleHead>
	<ArticleBody>
		<Row class="sm:container mx-auto px-4 grid lg:grid-cols-6 pb-16">
			<Column class="lg:col-start-2 col-span-4">
				<Section name="blog">
					<SectionCopy classes="my-4">
						Bobby’s presentation on Event Modeling at <A
							class="transition duration-300 ease-in"
							href="https://kafka-summit.org/">Kafka Summit 2020</A
						> mere days after the launching of the oNote product. In the talk, he builds an Event Model
						of a simple information system (including wire-framing the UI/UX experience), explores how
						to proceed from model to implementation using Kafka, shows you how to jump-start the implementation
						by generating code directly from the Event Model, and also how to track and measure the work
						of implementation by generating tasks directly from the Event Model.
					</SectionCopy>
				</Section>
			</Column>
		</Row>
	</ArticleBody>
</Article>
